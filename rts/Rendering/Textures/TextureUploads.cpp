/* This file is part of the Recoil engine (GPL v2 or later), see LICENSE.html */

#include "TextureUploads.h"

#include <algorithm>
#include <deque>
#include <limits>

#include "Rendering/GL/myGL.h"
#include "Rendering/GL/TexBind.h"
#include "Rendering/Textures/Bitmap.h"
#include "System/Config/ConfigHandler.h"
#include "System/Misc/SpringTime.h"
#include "System/Misc/TracyDefs.h"

CONFIG(float, TextureUploadRate)
	.defaultValue(128.0f)
	.minimumValue(0.0f)
	.description("MiB a second of texture data uploaded for textures made over several frames (model maps of a model loaded while the game runs, named textures queued with PreloadTexture): each drawn frame uploads its share of a typical frame's time (the shorter of the last frame's and the recent average, at most a thirtieth of a second), so the uploads keep pace with the GPU at any frame rate and a long frame does not make the next one longer. A block-compressed map goes up coarsest level first, a larger level in rows over the next frames. 0 makes each queued texture whole in the next frame.");

namespace TextureUploads {
namespace {

// A level this small goes up whole even past the budget: the coarse levels of a map are a few KiB.
constexpr size_t SMALL_LEVEL_BYTES = 64 * 1024;

struct Job {
	Ticket ticket = 0;
	Request request;
	uint32_t texID = 0;
	bool ownsTexture = false; // made by the job: deleted if the job is dropped
	bool started = false;
	bool complete = false;

#ifndef HEADLESS
	// a block-compressed 2D image with levels, in steps; the next level to make, coarsest first,
	// and the block rows of it already up
	bool staged = false;
	uint32_t skip = 0;
	uint32_t levels = 0;
	int32_t level = -1;
	uint32_t rowsDone = 0;

	const nv_dds::CSurface& Source(uint32_t lvl) const {
		const nv_dds::CDDSImage& dds = request.image->ddsimage;
		const uint32_t at = lvl + skip;
		return (at == 0)? static_cast<const nv_dds::CSurface&>(dds.get_image(0)): dds.get_mipmap(at - 1);
	}
#endif

	// Uploads up to `budget` bytes (at least one piece when `mayExceed`); the bytes it uploaded.
	size_t Step(size_t budget, bool mayExceed);
	void Finish();
	void Begin();
};

std::deque<Job> jobs;
Ticket nextTicket = 1;
spring_time lastUpdate;
float frameSecs = 1.0f / 60.0f; // recent frames' average time

// This frame's bytes: the rate's share of a typical frame. A fixed amount a frame outran the GPU at a
// high frame rate (4 MiB a frame at 500 frames a second left the GPU 10 ms behind); and a share of
// the last frame alone fed itself: a long frame (the one that makes a model) gave the next a large
// share, which made it long too.
size_t FrameBudget(float lastSecs)
{
	const float mibPerSec = configHandler->GetFloat("TextureUploadRate");
	if (mibPerSec <= 0.0f)
		return std::numeric_limits<size_t>::max();

	const float secs = std::min({lastSecs, frameSecs, 1.0f / 30.0f});
	return static_cast<size_t>(mibPerSec * secs * 1024.0f * 1024.0f);
}

size_t ImageBytes(const CBitmap& image)
{
#ifndef HEADLESS
	if (image.compressed)
		return image.ddsimage.get_size();
#endif
	return image.GetMemSize();
}

void Job::Begin()
{
	started = true;
	texID = request.texID;
#ifndef HEADLESS
	const CBitmap& image = *request.image;
	const nv_dds::CDDSImage& dds = image.ddsimage;

	// in steps: a block-compressed 2D image with at least two levels left after the skip,
	// stored as whole 4x4 blocks
	if (image.compressed && dds.is_valid() && dds.get_type() == nv_dds::TextureFlat && dds.is_compressed() && dds.get_num_mipmaps() > 0) {
		const uint32_t wanted = TextureMipSkipLevels(std::max(dds.get_width(), dds.get_height()), request.mipFloor, request.mipSkip);
		skip = std::min(wanted, dds.get_num_mipmaps());
		levels = dds.get_num_mipmaps() + 1 - skip;

		bool whole = levels >= 2;
		for (uint32_t lvl = 0; whole && lvl < levels; ++lvl) {
			const nv_dds::CSurface& s = Source(lvl);
			const uint32_t blockRows = (s.get_height() + 3) / 4;
			whole = (s.get_size() > 0 && s.get_size() % blockRows == 0);
		}
		staged = whole;
	}

	if (!staged)
		return;

	if (texID == 0) {
		glGenTextures(1, &texID);
		ownsTexture = true;
	}

	auto binding = GL::TexBind(GL_TEXTURE_2D, texID);
	// the storage of every level at once, filled by Step: a driver lays the chain out once, rather
	// than again (copying what is there) each time a finer level is defined
	for (uint32_t lvl = 0; lvl < levels; ++lvl) {
		const nv_dds::CSurface& s = Source(lvl);
		glTexImage2D(GL_TEXTURE_2D, lvl, dds.get_format(), s.get_width(), s.get_height(), 0, GL_RGBA, GL_UNSIGNED_BYTE, nullptr);
	}
	// drawable from the first level made: sampled between the finest level complete and the coarsest
	glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAX_LEVEL, static_cast<GLint>(levels - 1));
	glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_BASE_LEVEL, static_cast<GLint>(levels - 1));
	glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, request.tcp.GetMinFilter(dds.get_num_mipmaps()));
	level = static_cast<int32_t>(levels - 1);
	rowsDone = 0;
#endif
}

size_t Job::Step(size_t budget, bool mayExceed)
{
	RECOIL_DETAILED_TRACY_ZONE;
	if (!started)
		Begin();

#ifndef HEADLESS
	if (staged) {
		const uint32_t format = request.image->ddsimage.get_format();
		auto binding = GL::TexBind(GL_TEXTURE_2D, texID);
		size_t spent = 0;

		while (level >= 0) {
			const nv_dds::CSurface& s = Source(static_cast<uint32_t>(level));
			const uint32_t w = s.get_width();
			const uint32_t h = s.get_height();
			const uint32_t blockRows = (h + 3) / 4;
			const size_t rowBytes = s.get_size() / blockRows;
			const size_t left = budget - std::min(budget, spent);
			const bool first = (spent == 0 && mayExceed);

			if (rowsDone == 0) {
				const size_t size = s.get_size();
				if (size <= left || size <= SMALL_LEVEL_BYTES || (first && blockRows == 1)) {
					glCompressedTexSubImage2D(GL_TEXTURE_2D, level, 0, 0, w, h, format, size, s);
					spent += size;
					rowsDone = blockRows;
				} else {
					const uint32_t rows = static_cast<uint32_t>(std::min<size_t>(blockRows, left / rowBytes));
					if (rows == 0 && !first)
						break;

					// its rows; a compressed format's rows go in whole blocks
					const uint32_t n = std::max(1u, rows);
					glCompressedTexSubImage2D(GL_TEXTURE_2D, level, 0, 0, w, std::min(h, n * 4), format, n * rowBytes, s);
					spent += n * rowBytes;
					rowsDone = n;
				}
			} else {
				const uint32_t rows = static_cast<uint32_t>(std::min<size_t>(blockRows - rowsDone, left / rowBytes));
				if (rows == 0 && !first)
					break;

				const uint32_t n = std::max(1u, rows);
				const uint32_t y = rowsDone * 4;
				const unsigned char* src = static_cast<const unsigned char*>(s) + rowsDone * rowBytes;
				glCompressedTexSubImage2D(GL_TEXTURE_2D, level, 0, y, w, std::min(h - y, n * 4), format, n * rowBytes, src);
				spent += n * rowBytes;
				rowsDone += n;
			}

			if (rowsDone < blockRows)
				break;

			// complete: sampled from here
			glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_BASE_LEVEL, level);
			level--;
			rowsDone = 0;
		}

		if (level < 0)
			Finish();

		return spent;
	}
#endif

	const size_t bytes = ImageBytes(*request.image);
	if (bytes > budget && !mayExceed)
		return 0;

	// finished by makeWhole itself
	complete = true;
	ownsTexture = false;
	texID = request.makeWhole(*request.image, request.texID);
	return bytes;
}

void Job::Finish()
{
#ifndef HEADLESS
	if (staged) {
		const CBitmap& image = *request.image;
		const GL::TextureCreationParams& tcp = request.tcp;
		auto binding = GL::TexBind(GL_TEXTURE_2D, texID);

		// as CBitmap::CreateDDSTexture leaves a 2D DDS texture
		glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_BASE_LEVEL, 0);
		if (tcp.lodBias != 0.0f)
			glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_LOD_BIAS, tcp.lodBias);
		if (tcp.aniso > 0.0f)
			glTexParameterf(GL_TEXTURE_2D, GL_TEXTURE_MAX_ANISOTROPY_EXT, tcp.aniso);
		glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, tcp.GetMinFilter(image.ddsimage.get_num_mipmaps()));
	}
#endif

	complete = true;
	ownsTexture = false;
	if (request.done)
		request.done(texID);
}

// A job not complete. Jobs leave the queue in Update only: a done callback may complete or
// cancel another job while one is being stepped.
Job* Find(Ticket ticket)
{
	const auto it = std::find_if(jobs.begin(), jobs.end(), [ticket](const Job& job) { return job.ticket == ticket && !job.complete; });
	return (it == jobs.end())? nullptr: &*it;
}

} // namespace


Ticket Queue(Request&& request)
{
	Job& job = jobs.emplace_back();
	job.ticket = nextTicket++;
	job.request = std::move(request);
	return job.ticket;
}

void Update()
{
	RECOIL_DETAILED_TRACY_ZONE;
	const spring_time now = spring_gettime();
	const float lastSecs = std::clamp((now - lastUpdate).toSecsf(), 0.0f, 0.1f);
	lastUpdate = now;
	frameSecs += (lastSecs - frameSecs) * 0.1f;
	if (jobs.empty())
		return;

	const size_t budget = FrameBudget(lastSecs);
	size_t spent = 0;

	// in order; the first may go past the budget by one piece, so every frame makes progress
	for (size_t i = 0; i < jobs.size() && spent < budget; ++i) {
		if (!jobs[i].complete)
			spent += jobs[i].Step(budget - spent, spent == 0);
	}

	// a done callback may have queued more: drop the complete ones only
	std::erase_if(jobs, [](const Job& job) { return job.complete; });
}

bool Complete(Ticket ticket)
{
	RECOIL_DETAILED_TRACY_ZONE;
	Job* job = Find(ticket);
	if (job == nullptr)
		return false;

	// references into the deque survive what a done callback queues
	while (!job->complete)
		job->Step(std::numeric_limits<size_t>::max(), true);

	return true;
}

bool Cancel(Ticket ticket)
{
	Job* job = Find(ticket);
	if (job == nullptr)
		return false;

	if (job->ownsTexture && job->texID != 0)
		glDeleteTextures(1, &job->texID);

	job->ownsTexture = false;
	job->complete = true;
	return true;
}

size_t NumQueued()
{
	return std::count_if(jobs.begin(), jobs.end(), [](const Job& job) { return !job.complete; });
}

void Kill()
{
	for (const Job& job: jobs) {
		if (job.ownsTexture && job.texID != 0)
			glDeleteTextures(1, &job.texID);
	}

	jobs.clear();
}

} // namespace TextureUploads
