/* This file is part of the Recoil engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <functional>
#include <memory>

#include "Rendering/Textures/TextureCreationParams.hpp"

class CBitmap;

/**
 * Textures made from decoded images over several drawn frames, within one upload rate that every
 * queued texture shares (TextureUploadRate, MiB a second, each frame its share of the time since
 * the last). An upload of a large map from client
 * memory costs the frame that makes it in proportion to its size (a 4096 x 4096 DXT5 map with its
 * levels is 21 MiB); two in one frame, or one beside a model's first load, stalled it.
 *
 * A block-compressed 2D image with mipmaps goes up coarsest level first, and a level larger than
 * what is left of the budget goes in rows of blocks over the next frames. The texture can be drawn
 * at every step: its base level is the finest one complete, so it sharpens as levels arrive
 * (mip streaming, as Unreal's and Unity's texture streaming do). Any other image is made whole, in
 * a frame of its own when it is larger than the budget.
 *
 * Main thread only. Jobs go in the order queued.
 */
namespace TextureUploads {
	using Ticket = uint64_t;

	struct Request {
		/// the decoded image, held until its texture is made
		std::shared_ptr<const CBitmap> image;
		/// the texture to make it in (its earlier contents replaced); 0 makes a new one
		uint32_t texID = 0;
		/// for a block-compressed image: TextureMipSkip and TextureMipFloor, as read when the image
		/// was queued (a texture made in steps does not depend on the settings at each step)
		uint32_t mipSkip = 0;
		uint32_t mipFloor = 1;
		/// filtering of the finished texture, as CBitmap::CreateDDSTexture applies it
		GL::TextureCreationParams tcp;
		/// makes an image that does not go up in steps, in one call, and finishes it as `done` would
		/// (done is not called for it); returns its texture
		std::function<uint32_t(const CBitmap& image, uint32_t texID)> makeWhole;
		/// a texture made in steps is complete
		std::function<void(uint32_t texID)> done;
	};

	/// Queues a texture; the returned ticket names it to Complete and Cancel.
	Ticket Queue(Request&& request);
	/// Uploads the frame's share of the rate. Once a drawn frame.
	void Update();
	/// Makes a queued texture now, whole (it was asked for before its turn). False when not queued.
	bool Complete(Ticket ticket);
	/// Drops a queued texture: a texture made by the job is deleted, one given to it is left as it is.
	bool Cancel(Ticket ticket);
	/// Textures queued and not complete.
	size_t NumQueued();
	/// Drops every queued texture, as Cancel.
	void Kill();
}
