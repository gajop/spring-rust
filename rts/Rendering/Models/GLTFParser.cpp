/* This file is part of the Recoil engine (GPL v2 or later), see LICENSE.html */

#include <array>
#include <cstring>
#include <optional>
#include <variant>
#include <vector>
#include <numeric>
#include <algorithm>
#include <unordered_set>
#include <unordered_map>

#include "GLTFParser.h"
#include "3DModel.hpp"
#include "3DModelLog.h"
#include "GLTFModelAxisConversion.hpp"
#include "ModelUtils.h"

#include "Lua/LuaParser.h"
#include "Rendering/Textures/S3OTextureHandler.h"
#include "System/FileSystem/FileHandler.h"
#include "System/FileSystem/FileSystem.h"
#include "System/Misc/TracyDefs.h"
#include "System/Exceptions.h"
#include "System/UnorderedSet.hpp"
#include "System/Threading/ThreadPool.h"

#if defined(__linux__) || defined(__APPLE__)
	#include <fcntl.h>
	#include <sys/mman.h>
	#include <sys/stat.h>
	#include <unistd.h>
#endif

#include <fastgltf/core.hpp>
#include <fastgltf/tools.hpp>
#include <fastgltf/math.hpp>
#include <simdjson.h>


namespace Impl {
	Transform TRStoTransform(const fastgltf::TRS& trs) {
		assert(float3(trs.scale.x(), trs.scale.y(), trs.scale.z()) == float3(trs.scale.x()));
		return Transform{
			CQuaternion{ trs.rotation.x(), trs.rotation.y(), trs.rotation.z(), trs.rotation.w()},
			float3{ trs.translation.x(), trs.translation.y(), trs.translation.z() },
			trs.scale.x()
		};
	}
	Transform MatrixToTransform(const fastgltf::math::fmat4x4& matrix) {
		CMatrix44f mat;
		memcpy(&mat.m[0], matrix.data(), sizeof(CMatrix44f));
		auto [t, r, s] = mat.DecomposeIntoTRS();
		assert(s == float3(s));
		return Transform{ r, t, s.x };
	}

	Transform GetNodeTransform(
		const fastgltf::Node& node,
		gltfmodel::SourceConvention sourceConvention
	) {
		const Transform sourceTransform = fastgltf::visit_exhaustive(fastgltf::visitor{
			[](const fastgltf::math::fmat4x4& matrix) {
				return Impl::MatrixToTransform(matrix);
			},
			[](const fastgltf::TRS& trs) {
				return Impl::TRStoTransform(trs);
			}
		}, node.transform);

		return gltfmodel::ToEngineSpace(sourceTransform, sourceConvention);
	}

	// The model file's bytes for fastgltf, read in place. Only the JSON is copied, into a buffer
	// with the padding simdjson reads past its end.
	class FileBytes final : public fastgltf::GltfDataGetter {
	public:
		FileBytes(const std::byte* bytes_, size_t size_): bytes(bytes_), size(size_) {}

		void read(void* ptr, size_t count) override {
			const size_t n = std::min(count, size - idx);
			// the binary chunk is read where it is (MapChunk): nothing to copy
			if (ptr != bytes + idx) {
				std::memcpy(ptr, bytes + idx, n);
				std::memset(static_cast<std::byte*>(ptr) + n, 0, count - n);
			}
			idx += n;
		}
		fastgltf::span<std::byte> read(size_t count, size_t padding) override {
			const size_t n = std::min(count, size - idx);
			padded.assign(count + padding, std::byte{0});
			std::memcpy(padded.data(), bytes + idx, n);
			idx += n;
			return fastgltf::span<std::byte>(padded.data(), count);
		}
		void reset() override { idx = 0; }
		size_t bytesRead() override { return idx; }
		size_t totalSize() override { return size; }

		/// Where the next `count` bytes are, when the file has them.
		const std::byte* Here(size_t count) const { return (count <= size - idx)? bytes + idx: nullptr; }

	private:
		const std::byte* bytes;
		size_t size;
		size_t idx = 0;
		std::vector<std::byte> padded;
	};

	// What the parse callbacks are given: the model's optional parameters (scene extras) and the
	// file, whose GLB binary chunk fastgltf takes in place instead of copying it (MapChunk).
	struct ParseContext {
		ModelUtils::ModelParams* params = nullptr;
		FileBytes* file = nullptr;
		fastgltf::span<const std::byte> glb;
	};

	fastgltf::BufferInfo MapChunk(std::uint64_t chunkSize, void* userPointer) {
		auto* ctx = static_cast<ParseContext*>(userPointer);
		const std::byte* at = ctx->file->Here(static_cast<size_t>(chunkSize));
		if (at == nullptr)
			return {nullptr, 0};

		ctx->glb = fastgltf::span<const std::byte>(at, static_cast<size_t>(chunkSize));
		// fastgltf reads the chunk into this address, which FileBytes::read recognises: never written
		return {const_cast<std::byte*>(at), 0};
	}

	// Buffer views as fastgltf's DefaultBufferDataAdapter gives them, the GLB binary chunk from the
	// file in place (a CustomBuffer, MapChunk).
	struct BufferAdapter {
		fastgltf::span<const std::byte> glb;

		fastgltf::span<const std::byte> operator()(const fastgltf::Asset& asset, size_t bufferViewIdx) const {
			const auto& view = asset.bufferViews[bufferViewIdx];
			if (!std::holds_alternative<fastgltf::sources::CustomBuffer>(asset.buffers[view.bufferIndex].data))
				return fastgltf::DefaultBufferDataAdapter{}(asset, bufferViewIdx);
			if (view.byteOffset > glb.size() || view.byteLength > glb.size() - view.byteOffset)
				throw content_error("A GLTF model has a buffer view outside its binary chunk");

			return glb.subspan(view.byteOffset, view.byteLength);
		}
	};

	// A model file's bytes: mapped where the file is on disk (in a directory archive or the raw
	// filesystem), so its pages come from the page cache without a copy into new memory; read
	// through the VFS otherwise (an archive).
	class ModelFile {
	public:
		explicit ModelFile(const std::string& path) {
#if defined(__linux__) || defined(__APPLE__)
			if (const std::string absPath = CFileHandler::GetFileAbsolutePath(path, SPRING_VFS_ZIP); !absPath.empty()) {
				const int fd = open(absPath.c_str(), O_RDONLY);
				struct stat st {};
				if (fd >= 0 && fstat(fd, &st) == 0 && st.st_size > 0) {
	#if defined(__linux__)
					constexpr int flags = MAP_PRIVATE | MAP_POPULATE;
	#else
					constexpr int flags = MAP_PRIVATE;
	#endif
					void* at = mmap(nullptr, static_cast<size_t>(st.st_size), PROT_READ, flags, fd, 0);
					if (at != MAP_FAILED) {
						mapped = at;
						size = static_cast<size_t>(st.st_size);
						data = static_cast<const std::byte*>(at);
					}
				}
				if (fd >= 0)
					close(fd);
				if (mapped != nullptr)
					return;
			}
#endif
			CFileHandler file(path, SPRING_VFS_ZIP);
			if (!file.IsBuffered()) {
				const auto fs = file.FileSize();
				if (fs <= 0)
					throw content_error("A GLTF model has invalid size of " + std::to_string(fs));

				buffer.resize(fs, 0);
				file.Read(buffer.data(), buffer.size());
			}
			else {
				buffer = std::move(file.GetBuffer());
			}
			data = reinterpret_cast<const std::byte*>(buffer.data());
			size = buffer.size();
		}
		~ModelFile() {
#if defined(__linux__) || defined(__APPLE__)
			if (mapped != nullptr)
				munmap(mapped, size);
#endif
		}
		ModelFile(const ModelFile&) = delete;
		ModelFile& operator=(const ModelFile&) = delete;

		const std::byte* data = nullptr;
		size_t size = 0;

	private:
		std::vector<uint8_t> buffer;
		void* mapped = nullptr;
	};

	// Vertices converted by one task of a parallel geometry read: the per-vertex work of a large
	// mesh (a ship's hull is 100k+ vertices) runs on the worker threads with the caller, since a
	// model nobody queued is read in the frame that makes its first unit.
	static constexpr size_t VERTS_PER_TASK = 16384;

	// Runs f(begin, end) over [0, count) in chunks of `chunk`, on the worker threads when there are
	// several chunks; each element is written by exactly one call, so the result does not depend on
	// the threads.
	template<typename F>
	void ForChunks(size_t count, size_t chunk, F&& f) {
		const size_t numChunks = (count + chunk - 1) / chunk;
		if (numChunks <= 1) {
			if (count > 0)
				f(size_t(0), count);
			return;
		}
		for_mt(0, static_cast<int>(numChunks), [&](const int c) {
			const size_t b = static_cast<size_t>(c) * chunk;
			f(b, std::min(count, b + chunk));
		});
	}

	// a colour component as the general path stores it
	inline uint8_t ColorByte(float v) { return static_cast<uint8_t>(std::clamp(v, 0.0f, 1.0f) * 255.0f + 0.5f); }

	// An accessor of `N` floats an element, read in place from its buffer: float components, not
	// normalized, not sparse. Anything else is read through fastgltf's converting iteration.
	template<size_t N>
	struct FloatStream {
		const std::byte* data = nullptr;
		size_t stride = 0;

		std::array<float, N> operator[](size_t i) const {
			std::array<float, N> v;
			std::memcpy(v.data(), data + i * stride, N * sizeof(float));
			return v;
		}
	};

	template<size_t N>
	std::optional<FloatStream<N>> FloatStreamOf(const fastgltf::Asset& asset, const fastgltf::Accessor& accessor, const BufferAdapter& buffers) {
		if (!accessor.bufferViewIndex.has_value() || accessor.sparse.has_value() || accessor.normalized)
			return std::nullopt;
		if (accessor.componentType != fastgltf::ComponentType::Float || fastgltf::getNumComponents(accessor.type) != N)
			return std::nullopt;

		const auto& view = asset.bufferViews[*accessor.bufferViewIndex];
		const auto bytes = buffers(asset, *accessor.bufferViewIndex);
		const size_t stride = view.byteStride.value_or(N * sizeof(float));
		if (stride < N * sizeof(float) || bytes.data() == nullptr)
			return std::nullopt;
		if (accessor.count > 0 && accessor.byteOffset + (accessor.count - 1) * stride + N * sizeof(float) > bytes.size())
			return std::nullopt;

		return FloatStream<N>{bytes.data() + accessor.byteOffset, stride};
	}

	// Reads a primitive of a mesh that is not skinned and has its normals and tangents, the
	// common case, with each vertex read once and the vertices split among the worker threads.
	// The result is that of the general path below (same operations in the same order for each
	// vertex, attributes applied in the file's order); false when the primitive needs the
	// general path (skin weights, generated normals or tangents, an attribute not stored as floats).
	template<typename Primitive>
	bool ReadPrimitiveInPlace(
		const fastgltf::Asset& asset,
		const Primitive& prim,
		std::vector<SVertexData>& verts,
		std::vector<uint32_t>& indcs,
		size_t nodeIdx,
		const BufferAdapter& buffers
	) {
		enum class Attr { Position, Normal, Tex0, Tex1, Tangent, Color3, Color4 };
		// a colour may be stored as normalized integers: converted by fastgltf, element by element
		struct Read { Attr attr; FloatStream<4> stream; const fastgltf::Accessor* accessor; };

		std::vector<Read> reads;
		size_t count = 0;
		bool seenNormal = false;
		bool seenTangents = false;
		bool seenTex2 = false;

		if (prim.type != fastgltf::PrimitiveType::Triangles || !prim.indicesAccessor.has_value())
			return false;

		for (const auto* it = prim.attributes.cbegin(); it != prim.attributes.cend(); ++it) {
			const auto& accessor = asset.accessors[it->accessorIndex];
			if (it == prim.attributes.cbegin())
				count = accessor.count;
			if (accessor.count != count)
				return false;
			// the general path skips an attribute without a buffer view, as here
			if (!accessor.bufferViewIndex.has_value())
				continue;

			const auto add = [&]<size_t N>(Attr attr) {
				const auto stream = FloatStreamOf<N>(asset, accessor, buffers);
				if (!stream.has_value())
					return false;
				reads.push_back({attr, FloatStream<4>{stream->data, stream->stride}, &accessor});
				return true;
			};
			const auto addColor = [&]() {
				if (accessor.type != fastgltf::AccessorType::Vec3 && accessor.type != fastgltf::AccessorType::Vec4)
					return true; // the general path ignores other shapes too
				reads.push_back({(accessor.type == fastgltf::AccessorType::Vec3)? Attr::Color3: Attr::Color4, {}, &accessor});
				return true;
			};

			switch (hashString(it->name.c_str(), it->name.length())) {
				case hashString("POSITION"  ): { if (!add.template operator()<3>(Attr::Position)) return false; } break;
				case hashString("NORMAL"    ): { if (!add.template operator()<3>(Attr::Normal  )) return false; seenNormal   = true; } break;
				case hashString("TEXCOORD_0"): { if (!add.template operator()<2>(Attr::Tex0    )) return false; } break;
				case hashString("TEXCOORD_1"): { if (!add.template operator()<2>(Attr::Tex1    )) return false; seenTex2     = true; } break;
				case hashString("TANGENT"   ): { if (!add.template operator()<4>(Attr::Tangent )) return false; seenTangents = true; } break;
				case hashString("COLOR_0"   ): { if (!addColor()) return false; } break;
				case hashString("JOINTS_0"  ):
				case hashString("JOINTS_1"  ):
				case hashString("WEIGHTS_0" ):
				case hashString("WEIGHTS_1" ):
					return false;
				default:
					break;
			}
		}

		if (!seenNormal || !seenTangents)
			return false;

		const auto& indxAccessor = asset.accessors[*prim.indicesAccessor];
		if (!indxAccessor.bufferViewIndex.has_value() || indxAccessor.sparse.has_value())
			return false;

		// a vertex with no skin weights is bound to its node alone (the general path's result)
		SVertexData boneVert;
		{
			std::array<std::pair<uint16_t, float>, 8> weights;
			weights.fill(std::make_pair(SVertexData::INVALID_BONEID, 0.0f));
			weights[0] = std::make_pair(static_cast<uint16_t>(nodeIdx), 1.0f);
			boneVert.SetBones(weights);
		}

		const size_t prevVertSize = verts.size();
		const size_t prevIndcSize = indcs.size();
		verts.resize(prevVertSize + count);
		indcs.resize(prevIndcSize + indxAccessor.count);

		SVertexData* out = verts.data() + prevVertSize;
		ForChunks(count, VERTS_PER_TASK, [&](size_t b, size_t e) {
			for (size_t i = b; i < e; ++i) {
				SVertexData& v = out[i];
				for (const Read& read: reads) {
					const std::byte* src = read.stream.data + i * read.stream.stride;
					float f[4];
					switch (read.attr) {
						case Attr::Position: { std::memcpy(f, src, 3 * sizeof(float)); v.pos = float3{f[0], f[1], f[2]}; } break;
						case Attr::Normal  : { std::memcpy(f, src, 3 * sizeof(float)); v.normal = float3{f[0], f[1], f[2]}.ANormalize(); } break;
						// note the UV.y is inverted, as in the general path
						case Attr::Tex0    : { std::memcpy(f, src, 2 * sizeof(float)); v.texCoords[0] = float2(f[0], 1.0f - f[1]); } break;
						case Attr::Tex1    : { std::memcpy(f, src, 2 * sizeof(float)); v.texCoords[1] = float2(f[0], 1.0f - f[1]); } break;
						case Attr::Tangent : {
							std::memcpy(f, src, 4 * sizeof(float));
							v.sTangent = float3{f[0], f[1], f[2]}.ANormalize();
							v.tTangent = f[3] * v.normal.cross(v.sTangent).ANormalize();
						} break;
						case Attr::Color3  : {
							const auto c = fastgltf::getAccessorElement<fastgltf::math::fvec3>(asset, *read.accessor, i, buffers);
							v.color = {{ ColorByte(c.x()), ColorByte(c.y()), ColorByte(c.z()), 255 }};
						} break;
						case Attr::Color4  : {
							const auto c = fastgltf::getAccessorElement<fastgltf::math::fvec4>(asset, *read.accessor, i, buffers);
							v.color = {{ ColorByte(c.x()), ColorByte(c.y()), ColorByte(c.z()), ColorByte(c.w()) }};
						} break;
					}
				}
				if (!seenTex2)
					v.texCoords[1] = v.texCoords[0];

				v.boneIDsLow  = boneVert.boneIDsLow;
				v.boneIDsHigh = boneVert.boneIDsHigh;
				v.boneWeights = boneVert.boneWeights;
			}
		});

		// indices, as fastgltf converts them (unsigned byte, short or int), offset to this
		// primitive's first vertex
		const auto indxBytes = buffers(asset, *indxAccessor.bufferViewIndex);
		const size_t indxSize = fastgltf::getElementByteSize(indxAccessor.type, indxAccessor.componentType);
		const size_t indxStride = asset.bufferViews[*indxAccessor.bufferViewIndex].byteStride.value_or(indxSize);
		const std::byte* indxData = indxBytes.data() + indxAccessor.byteOffset;
		const uint32_t indxOffset = static_cast<uint32_t>(prevVertSize);
		uint32_t* indxOut = indcs.data() + prevIndcSize;

		if (indxAccessor.count > 0 && indxAccessor.byteOffset + (indxAccessor.count - 1) * indxStride + indxSize > indxBytes.size())
			throw content_error("A GLTF model has an index accessor larger than its buffer view");

		const auto readIndices = [&]<typename T>() {
			ForChunks(indxAccessor.count, VERTS_PER_TASK * 4, [&](size_t b, size_t e) {
				for (size_t i = b; i < e; ++i) {
					T index;
					std::memcpy(&index, indxData + i * indxStride, sizeof(T));
					indxOut[i] = static_cast<uint32_t>(index) + indxOffset;
				}
			});
		};
		switch (indxAccessor.componentType) {
			case fastgltf::ComponentType::UnsignedByte : { readIndices.template operator()<uint8_t >(); } break;
			case fastgltf::ComponentType::UnsignedShort: { readIndices.template operator()<uint16_t>(); } break;
			case fastgltf::ComponentType::UnsignedInt  : { readIndices.template operator()<uint32_t>(); } break;
			default: {
				// not a valid index type: let the general path report it as it does
				verts.resize(prevVertSize);
				indcs.resize(prevIndcSize);
				return false;
			}
		}

		return true;
	}

	template<typename PrimContainer>
	void ReadGeometryData(
		const fastgltf::Asset& asset,
		const PrimContainer& primitives,
		std::vector<SVertexData>& verts,
		std::vector<uint32_t>& indcs,
		size_t nodeIdx,
		gltfmodel::SourceConvention sourceConvention,
		const BufferAdapter& buffers,
		const fastgltf::Skin* skinPtr = nullptr
	) {
			const size_t firstVertSize = verts.size();

			for (const auto& prim : primitives) {
				if (skinPtr == nullptr && ReadPrimitiveInPlace(asset, prim, verts, indcs, nodeIdx, buffers))
					continue;

				const size_t prevVertSize = verts.size();
				const size_t prevIndcSize = indcs.size();

				bool seenNormal = false;
				bool seenTangents = false;
				bool seenTex2 = false;

				std::vector<std::array<std::pair<uint16_t, float>, 8>> vertexWeights;

				if (prim.type != fastgltf::PrimitiveType::Triangles) {
					throw content_error("A GLTF model has invalid primitive type " + std::to_string(static_cast<uint32_t>(prim.type)));
				}

				for (const auto* primAttIt = prim.attributes.cbegin(); primAttIt != prim.attributes.cend(); ++primAttIt) {
					auto& accessor = asset.accessors[primAttIt->accessorIndex];

					if (primAttIt == prim.attributes.cbegin()) {
						verts.resize(prevVertSize + accessor.count);
						vertexWeights.resize(accessor.count);
						for (auto& weights : vertexWeights) {
							weights.fill(std::make_pair(SVertexData::INVALID_BONEID, 0.0f));
						}
					}

					if (!accessor.bufferViewIndex.has_value())
						continue;

					switch (hashString(primAttIt->name.c_str(), primAttIt->name.length()))
					{
					case hashString("POSITION"): {
						fastgltf::iterateAccessorWithIndex<fastgltf::math::fvec3>(asset, accessor, [&](const auto& val, std::size_t idx) {
							verts[prevVertSize + idx].pos = float3{ val.x(), val.y(), val.z() };
						}, buffers);
					} break;
					case hashString("NORMAL"): {
						fastgltf::iterateAccessorWithIndex<fastgltf::math::fvec3>(asset, accessor, [&](const auto& val, std::size_t idx) {
							verts[prevVertSize + idx].normal = float3{ val.x(), val.y(), val.z() }.ANormalize();
						}, buffers);
						seenNormal = true;
					} break;
					// note the UV.y is inverted!!!
					case hashString("TEXCOORD_0"): {
						fastgltf::iterateAccessorWithIndex<fastgltf::math::fvec2>(asset, accessor, [&](const auto& val, std::size_t idx) {
							verts[prevVertSize + idx].texCoords[0] = float2(val.x(), 1.0f - val.y());
						}, buffers);
					} break;
					case hashString("TEXCOORD_1"): {
						fastgltf::iterateAccessorWithIndex<fastgltf::math::fvec2>(asset, accessor, [&](const auto& val, std::size_t idx) {
							verts[prevVertSize + idx].texCoords[1] = float2(val.x(), 1.0f - val.y());
						}, buffers);
						seenTex2 = true;
					} break;
					case hashString("TANGENT"): {
						fastgltf::iterateAccessorWithIndex<fastgltf::math::fvec4>(asset, accessor, [&](const auto& val, std::size_t idx) {
							verts[prevVertSize + idx].sTangent = float3{ val.x(), val.y(), val.z() }.ANormalize();
							verts[prevVertSize + idx].tTangent = val.w() * verts[prevVertSize + idx].normal.cross(verts[prevVertSize + idx].sTangent).ANormalize();
						}, buffers);
						seenTangents = true;
					} break;
					case hashString("COLOR_0"): {
						const auto ToByte = [](float v) { return static_cast<uint8_t>(std::clamp(v, 0.0f, 1.0f) * 255.0f + 0.5f); };
						if (accessor.type == fastgltf::AccessorType::Vec3) {
							fastgltf::iterateAccessorWithIndex<fastgltf::math::fvec3>(asset, accessor, [&](const auto& val, std::size_t idx) {
								verts[prevVertSize + idx].color = { ToByte(val.x()), ToByte(val.y()), ToByte(val.z()), 255 };
							}, buffers);
						} else if (accessor.type == fastgltf::AccessorType::Vec4) {
							fastgltf::iterateAccessorWithIndex<fastgltf::math::fvec4>(asset, accessor, [&](const auto& val, std::size_t idx) {
								verts[prevVertSize + idx].color = { ToByte(val.x()), ToByte(val.y()), ToByte(val.z()), ToByte(val.w()) };
							}, buffers);
						}
					} break;
					case hashString("JOINTS_0"): {
						assert(skinPtr);
						fastgltf::iterateAccessorWithIndex<fastgltf::math::uvec4>(asset, accessor, [&](const auto& val, std::size_t idx) {
							auto& vertexWeight = vertexWeights[idx];
							vertexWeight[0].first = static_cast<uint16_t>(skinPtr->joints[val.x()]);
							vertexWeight[1].first = static_cast<uint16_t>(skinPtr->joints[val.y()]);
							vertexWeight[2].first = static_cast<uint16_t>(skinPtr->joints[val.z()]);
							vertexWeight[3].first = static_cast<uint16_t>(skinPtr->joints[val.w()]);
						}, buffers);
					} break;
					case hashString("JOINTS_1"): {
						assert(skinPtr);
						fastgltf::iterateAccessorWithIndex<fastgltf::math::uvec4>(asset, accessor, [&](const auto& val, std::size_t idx) {
							auto& vertexWeight = vertexWeights[idx];
							vertexWeight[4].first = static_cast<uint16_t>(skinPtr->joints[val.x()]);
							vertexWeight[5].first = static_cast<uint16_t>(skinPtr->joints[val.y()]);
							vertexWeight[6].first = static_cast<uint16_t>(skinPtr->joints[val.z()]);
							vertexWeight[7].first = static_cast<uint16_t>(skinPtr->joints[val.w()]);
						}, buffers);
					} break;
					case hashString("WEIGHTS_0"): {
						fastgltf::iterateAccessorWithIndex<fastgltf::math::fvec4>(asset, accessor, [&](const auto& val, std::size_t idx) {
							auto& vertexWeight = vertexWeights[idx];
							vertexWeight[0].second = val.x();
							vertexWeight[1].second = val.y();
							vertexWeight[2].second = val.z();
							vertexWeight[3].second = val.w();
						}, buffers);
					} break;
					case hashString("WEIGHTS_1"): {
						fastgltf::iterateAccessorWithIndex<fastgltf::math::fvec4>(asset, accessor, [&](const auto& val, std::size_t idx) {
							auto& vertexWeight = vertexWeights[idx];
							vertexWeight[4].second = val.x();
							vertexWeight[5].second = val.y();
							vertexWeight[6].second = val.z();
							vertexWeight[7].second = val.w();
						}, buffers);
					} break;
					default:
						break;
					}
				}

			for (auto& vertexWeight : vertexWeights) {
				for (auto& w : vertexWeight) {
					if (w.second == 0.0f)
						w.first = SVertexData::INVALID_BONEID;
				}

				// stable, descending by (weight, bone); an insertion sort of eight needs no buffer
				for (size_t a = 1; a < vertexWeight.size(); ++a) {
					const auto w = vertexWeight[a];
					size_t b = a;
					for (; b > 0 && std::forward_as_tuple(w.second, w.first) > std::forward_as_tuple(vertexWeight[b - 1].second, vertexWeight[b - 1].first); --b)
						vertexWeight[b] = vertexWeight[b - 1];
					vertexWeight[b] = w;
				}

				// non-skinning case
				if (auto& [boneId, boneWeight] = vertexWeight.front(); boneId == SVertexData::INVALID_BONEID) {
					boneId = nodeIdx;
					boneWeight = 1.0f;
				}
			}

			for (size_t i = prevVertSize; i < verts.size(); ++i) {
				verts[i].SetBones(vertexWeights[i - prevVertSize]);
			}

			assert(prim.indicesAccessor.has_value());
			auto& accessor = asset.accessors[*prim.indicesAccessor];
			indcs.resize(prevIndcSize + accessor.count);

			assert(accessor.bufferViewIndex.has_value());
			// a primitive's indices count from its own first vertex
			fastgltf::iterateAccessorWithIndex<uint32_t>(asset, accessor, [&](std::uint32_t index, std::size_t idx) {
				indcs[prevIndcSize + idx] = index + static_cast<uint32_t>(prevVertSize);
			}, buffers);

			// certain post-processing
			if (!seenTex2)
				std::for_each(verts.begin() + prevVertSize, verts.end(), [](auto& vert) { vert.texCoords[1] = vert.texCoords[0]; });

			if (!seenNormal)
				ModelUtils::CalculateNormals(verts, indcs);

			if (!seenTangents)
				ModelUtils::CalculateTangents(verts, indcs);

		}

		ForChunks(verts.size() - firstVertSize, VERTS_PER_TASK, [&](size_t b, size_t e) {
			for (size_t i = firstVertSize + b; i < firstVertSize + e; ++i) {
				auto& vert = verts[i];
				vert.pos = gltfmodel::ToEngineSpace(vert.pos, sourceConvention);
				vert.normal = gltfmodel::ToEngineSpace(vert.normal, sourceConvention);
				vert.sTangent = gltfmodel::ToEngineSpace(vert.sTangent, sourceConvention);
				vert.tTangent = gltfmodel::ToEngineSpace(vert.tTangent, sourceConvention);
			}
		});
	}

	template<typename UM>
	void ReplaceNodeIndexWithPieceIndex(std::vector<SVertexData>& verts, const UM& nodeIdxToPieceIdx) {
		// most vertices of a piece name the same node: looked up once for a run of them
		int lastNode = -1;
		size_t lastPiece = 0;
		for (auto& vert : verts) {
			for (size_t wi = 0; wi < vert.boneIDsLow.size(); ++wi) {
				const auto nodeIdx = Skinning::GetBoneID(vert, wi);
				if (nodeIdx == INV_PIECE_NUM)
					continue;

				if (static_cast<int>(nodeIdx) != lastNode) {
					const auto pIt = nodeIdxToPieceIdx.find(nodeIdx);
					assert(pIt != nodeIdxToPieceIdx.end());
					lastNode = static_cast<int>(nodeIdx);
					lastPiece = pIt->second;
				}
				vert.boneIDsLow [wi] = static_cast<uint8_t>((lastPiece >> 0) & 0xFF);
				vert.boneIDsHigh[wi] = static_cast<uint8_t>((lastPiece >> 8) & 0xFF);
			}
		}
	}

	template<typename UM>
	void TransformSkinsToModelSpace(std::vector<SVertexData>& verts, size_t nodeIdx, const UM& transforms) {
		auto it = transforms.find(nodeIdx);
		assert(it != transforms.end());

		// Skins are expected to be in the model space anyway,
		// So this whole function is mostly a precaution
		if likely(it->second.IsIdentity())
			return;

		for (auto& vert : verts) {
			vert.TransformBy(it->second);
		}
	}

	void ParseSceneExtra(simdjson::dom::object* extras, std::size_t objectIndex, fastgltf::Category objectType, void* userPointer) {
		if (objectType != fastgltf::Category::Scenes)
			return;

		if (!extras)
			return;

		auto ParseFloat3 = [](const simdjson::dom::element& value) -> std::optional<float3> {
			if (!value.is_array() || value.get_array().size() != 3)
				return std::nullopt;

			float3 f3{};
			for (size_t i = 0; i < 3; ++i) {
				f3[i] = static_cast<float>(value.get_array().at(i).get_double());
			}

			return std::make_optional(std::move(f3));
		};

		auto& optionalModelParams = *static_cast<ParseContext*>(userPointer)->params;
		for (const auto& [key, value] : *extras) {
			switch (hashStringLower(key.data(), key.size())) {
			case hashString("tex1"): {
				if (value.is_string())
					optionalModelParams.texs[0] = value;
			} break;
			case hashString("tex2"): {
				if (value.is_string())
					optionalModelParams.texs[1] = value;
			} break;
			case hashString("midpos"): {
				optionalModelParams.relMidPos = ParseFloat3(value);
			} break;
			case hashString("mins"): {
				optionalModelParams.mins = ParseFloat3(value);
			} break;
			case hashString("maxs"): {
				optionalModelParams.maxs = ParseFloat3(value);
			} break;
			case hashString("height"): {
				optionalModelParams.height = static_cast<float>(value.get_double());
			} break;
			case hashString("radius"): {
				optionalModelParams.radius = static_cast<float>(value.get_double());
			} break;
			case hashString("fliptextures"): {
				if (value.is_bool())
					optionalModelParams.flipTextures = value;
			} break;
			case hashString("invertteamcolor"): {
				if (value.is_bool())
					optionalModelParams.invertTeamColor = value;
			} break;
			case hashString("s3ocompat"): {
				if (value.is_bool())
					optionalModelParams.s3oCompat = value;
			} break;
			default: {
				/*NO-OP*/
			} break;
			}
		}
	}

	void FindTextures(S3DModel* model, const fastgltf::Asset& asset, const std::string& modelBaseName, const ModelUtils::ModelParams& optionalModelParams)
	{
		std::string fullPath;
		for (int i = 0; i < 2; ++i) {
			if (optionalModelParams.texs[i].has_value()) {
				fullPath = "unittextures/" + *optionalModelParams.texs[i];
				if (CFileHandler::FileExists(fullPath, SPRING_VFS_ZIP_FIRST)) {
					model->texs[i] = fullPath;
					continue;
				}
			}
		}

		// TODO parse asset?
		// TODO guess the texture?
	}

	auto GetModelTransforms(const fastgltf::Asset& asset, std::size_t sceneIndex, gltfmodel::SourceConvention sourceConvention) {
		auto& scene = asset.scenes[sceneIndex];

		spring::unordered_map<size_t, Transform> transforms(asset.nodes.size());

		auto function = [&](auto& self, size_t nodeIdx, const Transform& parentTransform) -> void {
			const auto& node = asset.nodes[nodeIdx];

			const auto& [it, noDup] = transforms.emplace(
				nodeIdx,
				parentTransform * GetNodeTransform(node, sourceConvention)
			);
			assert(noDup);

			for (auto& child : node.children) {
				self(self, child, it->second);
			}
		};

		for (auto& node : scene.nodeIndices) {
			function(function, node, Transform{});
		}

		return transforms;
	}
}

void CGLTFParser::Load(S3DModel& model, const std::string& modelFilePath)
{
	RECOIL_DETAILED_TRACY_ZONE;
	LOG_SL(LOG_SECTION_MODEL, L_INFO, "Loading model: %s", modelFilePath.c_str());

	const std::string modelPath = FileSystem::GetDirectory(modelFilePath);
	const std::string modelName = FileSystem::GetBasename(modelFilePath);

	// read in place: the file mapped where it can be, and fastgltf given its binary chunk where
	// it is, not copies of either (a ship's model is 10 MiB and more, read in the frame that makes
	// its first unit when nobody queued it)
	const Impl::ModelFile modelFile(modelFilePath);
	Impl::FileBytes gltfFile(modelFile.data, modelFile.size);

	static constexpr auto PARSER_OPTION =
		fastgltf::Options::DontRequireValidAssetMember |
		//fastgltf::Options::DecomposeNodeMatrices | // most likely sync unsafe, however this doesn't mean the transformation type is always matrix
		fastgltf::Options::GenerateMeshIndices;

	static constexpr auto PARSER_CATEGORIES =
		fastgltf::Category::Buffers |
		fastgltf::Category::BufferViews |
		fastgltf::Category::Accessors |
		fastgltf::Category::Images | //?
		fastgltf::Category::Animations |
		fastgltf::Category::Meshes |
		fastgltf::Category::Skins |
		fastgltf::Category::Nodes |
		fastgltf::Category::Scenes |
		fastgltf::Category::Asset;

	fastgltf::Parser parser;

	ModelUtils::ModelParams optionalModelParams;
	Impl::ParseContext parseContext{&optionalModelParams, &gltfFile, {}};
	parser.setExtrasParseCallback(&Impl::ParseSceneExtra);
	parser.setBufferAllocationCallback(&Impl::MapChunk);
	parser.setUserPointer(&parseContext);

	auto maybeGltf = parser.loadGltf(gltfFile, modelPath, PARSER_OPTION, PARSER_CATEGORIES);
	if (auto error = maybeGltf.error(); error != fastgltf::Error::None) {
		throw content_error("Error loading GLTF file " + modelFilePath);
	}

	const auto& asset = maybeGltf.get();
	if (asset.scenes.empty()) {
		throw content_error("Error loading GLTF file " + modelFilePath);
	}
	const Impl::BufferAdapter buffers{parseContext.glb};

	//////// Lua metafile
	// load the lua metafile containing properties unique to Recoil models (must return a table)
	std::string metaFileName = modelFilePath + ".lua";

	// try again without the model file extension
	if (!CFileHandler::FileExists(metaFileName, SPRING_VFS_ZIP))
		metaFileName = modelPath + modelName + ".lua";

	if (!CFileHandler::FileExists(metaFileName, SPRING_VFS_ZIP)) {
		LOG_SL(LOG_SECTION_MODEL, L_INFO, "No meta-file found for the GLTF model '%s'.", metaFileName.c_str());
	}

	LuaParser metaFileParser(metaFileName, SPRING_VFS_ZIP, SPRING_VFS_ZIP);

	if (!metaFileParser.Execute()) {
		LOG_SL(LOG_SECTION_MODEL, L_INFO, "Error parsing the meta-file '%s': %s.", metaFileName.c_str(), metaFileParser.GetErrorLog().c_str());
	}

	// get the (root-level) model table
	const auto modelTable = metaFileParser.GetRoot();

	if (!modelTable.IsValid()) {
		LOG_SL(LOG_SECTION_MODEL, L_INFO, "No valid model metadata in '%s' or no meta-file", metaFileName.c_str());
	}

	// optionalModelParams will contain all non-empty data from the modelTable
	ModelUtils::GetModelParams(modelTable, optionalModelParams);

	// Load textures
	Impl::FindTextures(&model, asset, modelName, optionalModelParams);
	LOG_SL(LOG_SECTION_MODEL, L_INFO, "Loading textures. Tex1: '%s' Tex2: '%s'", model.texs[0].c_str(), model.texs[1].c_str());

	textureHandlerS3O.PreloadTexture(
		&model,
		optionalModelParams.flipTextures.value_or(false),
		optionalModelParams.invertTeamColor.value_or(false)
	);

	model.name = modelFilePath;
	model.type = MODELTYPE_ASS; // Revise?
	model.numPieces = 0;
	model.mins = DEF_MIN_SIZE;
	model.maxs = DEF_MAX_SIZE;

	// GLTF source coordinates are converted into the engine model frame at the
	// parser boundary. Keeping the synthetic scene root unrotated makes script
	// animation axes identical to those of S3O pieces.
	const auto sourceConvention = gltfmodel::GetSourceConvention(
		optionalModelParams.s3oCompat.value_or(false)
	);

	const auto defaultSceneIdx = asset.defaultScene.value_or(0);

	auto* rootPiece = AllocRootEmptyPiece(&model, asset, defaultSceneIdx, sourceConvention);
	model.FlattenPieceTree(rootPiece);

	// the pieces' meshes (a skinned mesh is read below, by the bones it is reparented to)
	for (auto* modelPiece: model.pieceObjects) {
		auto* piece = static_cast<GLTFPiece*>(modelPiece);
		if (piece->nodeIndex == GLTFPiece::INVALID_NODE_INDEX)
			continue;

		const auto& node = asset.nodes[piece->nodeIndex];
		if (!node.meshIndex.has_value())
			continue;

		const auto& mesh = asset.meshes[*node.meshIndex];
		Impl::ReadGeometryData(asset, mesh.primitives, piece->GetVerticesVec(), piece->GetIndicesVec(), piece->nodeIndex, sourceConvention, buffers, nullptr);
	}

	spring::unordered_map<size_t, size_t> nodeIdxToPieceIdx;
	for (size_t pi = 0; pi < model.pieceObjects.size(); ++pi) {
		const auto* piece = static_cast<const GLTFPiece*>(model.pieceObjects[pi]);
		if (piece->nodeIndex == GLTFPiece::INVALID_NODE_INDEX)
			continue;

		nodeIdxToPieceIdx[piece->nodeIndex] = pi;
	}

	// conceptually almost the same as AllocRootEmptyPiece + SetPieceMatrices
	// except this one doesn't ignore nodes with skinned meshes
	const auto modelTransforms = Impl::GetModelTransforms(asset, defaultSceneIdx, sourceConvention);

	std::vector<Skinning::SkinnedMesh> allSkinnedMeshes;

	for (size_t ni = 0; ni < asset.nodes.size(); ++ni) {
		const auto& node = asset.nodes[ni];
		if (!node.meshIndex.has_value())
			continue;

		if (!node.skinIndex.has_value())
			continue;

		const auto& skin = asset.skins[*node.skinIndex];
		const auto& mesh = asset.meshes[*node.meshIndex];
		auto& skinnedMesh = allSkinnedMeshes.emplace_back();

		Impl::ReadGeometryData(asset, mesh.primitives, skinnedMesh.verts, skinnedMesh.indcs, ni, sourceConvention, buffers, &skin);
		Impl::TransformSkinsToModelSpace(skinnedMesh.verts, ni, modelTransforms);
		Impl::ReplaceNodeIndexWithPieceIndex(skinnedMesh.verts, nodeIdxToPieceIdx);
	}

	// non-skinning case
	if (allSkinnedMeshes.empty()) {
		for (size_t pi = 0; pi < model.pieceObjects.size(); ++pi) {
			auto* piece = static_cast<GLTFPiece*>(model.pieceObjects[pi]);
			if (piece->nodeIndex == GLTFPiece::INVALID_NODE_INDEX)
				continue;

			if (piece->GetVerticesVec().empty())
				continue;

			Impl::ReplaceNodeIndexWithPieceIndex(piece->GetVerticesVec(), nodeIdxToPieceIdx);
		}
	}

	spring::unordered_set<size_t> allBones;
	for (const auto& skin : asset.skins) {
		for (size_t ji = 0; ji < skin.joints.size(); ++ji) {
			allBones.emplace(skin.joints[ji]);
		}
	}

	if (!allSkinnedMeshes.empty()) {
		// Skinning::<> code below needs correct bposeTransforms
		model.SetPieceMatrices();

		// if numMeshes >= numBones reparent the whole meshes
		// else reparent meshes per-triangle
		if (allSkinnedMeshes.size() >= allBones.size())
			Skinning::ReparentCompleteMeshesToBones(&model, allSkinnedMeshes);
		else
			Skinning::ReparentMeshesTrianglesToBones(&model, allSkinnedMeshes);
	}

	// will also calculate pieces / model bounding box
	ModelUtils::ApplyModelProperties(&model, optionalModelParams);

	ModelLog::LogModelProperties(model);
}

void CGLTFParser::Kill()
{
	RECOIL_DETAILED_TRACY_ZONE;
	LOG_L(L_INFO, "[GLTFParser::%s] allocated %u pieces", __func__, numPoolPieces);

	decltype(piecePool){}.swap(piecePool);
	numPoolPieces = 0;
}

GLTFPiece* CGLTFParser::AllocPiece()
{
	RECOIL_DETAILED_TRACY_ZONE;
	std::lock_guard<spring::mutex> lock(poolMutex);

	// lazily reserve pool here instead of during Init
	// this way games using only one model-type do not
	// cause redundant allocation
	if (piecePool.empty())
		piecePool.resize(MAX_MODEL_OBJECTS * AVG_MODEL_PIECES);

	if (numPoolPieces >= piecePool.size()) {
		throw std::bad_alloc();
		return nullptr;
	}

	return &piecePool[numPoolPieces++];
}

GLTFPiece* CGLTFParser::AllocRootEmptyPiece(
	S3DModel* model,
	const fastgltf::Asset& asset,
	size_t sceneIndex,
	gltfmodel::SourceConvention sourceConvention
)
{
	const auto& scene = asset.scenes[sceneIndex];

	auto* piece = AllocPiece();
	model->numPieces++;

	piece->SetParentModel(model);

	piece->SetBakedTransform(Transform{});
	piece->offset = float3{};
	piece->scale = 1.0f;


	piece->parent = nullptr;
	piece->name = scene.name;
	piece->children.reserve(scene.nodeIndices.size());

	for (const auto childNodeIndex : scene.nodeIndices) {
		auto* childPiece = LoadPiece(model, piece, asset, childNodeIndex, sourceConvention);
		if (childPiece)
			piece->children.push_back(childPiece);
	}

	return piece;
}


GLTFPiece* CGLTFParser::LoadPiece(
	S3DModel* model,
	GLTFPiece* parentPiece,
	const fastgltf::Asset& asset,
	size_t nodeIndex,
	gltfmodel::SourceConvention sourceConvention
)
{
	const auto& node = asset.nodes[nodeIndex];

	// skip skinned meshes (handled separately)
	if (node.skinIndex.has_value())
		return nullptr;

	auto* piece = AllocPiece();
	model->numPieces++;

	piece->SetParentModel(model);
	piece->parent = parentPiece;
	piece->name = node.name;
	piece->children.reserve(node.children.size());
	piece->nodeIndex = nodeIndex;

	Transform pieceTransform = Impl::GetNodeTransform(node, sourceConvention);
	
	auto bakedTransform = pieceTransform;
	// Piece translation and scale are stored separately; bakedTransform contains
	// only the artist-authored rest rotation expressed in engine axes.
	bakedTransform.t = float3{};
	bakedTransform.s = 1.0f;

	piece->SetBakedTransform(bakedTransform);
	piece->offset = pieceTransform.t;
	piece->scale = pieceTransform.s;

	for (const auto childNodeIndex : node.children) {
		auto* childPiece = LoadPiece(model, piece, asset, childNodeIndex, sourceConvention);
		if (childPiece)
			piece->children.push_back(childPiece);
	}

	// its geometry is read in Load, with the file's buffers
	return piece;
}
