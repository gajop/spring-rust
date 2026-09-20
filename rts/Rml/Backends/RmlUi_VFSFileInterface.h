// This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

#ifndef RMLUI_VFSFILEINTERFACE_H
#define RMLUI_VFSFILEINTERFACE_H

#include "RmlUi/Core/FileInterface.h"
#include "RmlUi/Core/Types.h"

#include <string>
#include <utility>

class VFSFileInterface : public Rml::FileInterface
{
public:
	VFSFileInterface();
	void SetModes(std::string modes) { vfsModes = std::move(modes); }
	Rml::FileHandle Open(const Rml::String& path) override;
	void Close(Rml::FileHandle file) override;
	size_t Read(void* buffer, size_t size, Rml::FileHandle file) override;
	bool Seek(Rml::FileHandle file, long offset, int origin) override;
	size_t Tell(Rml::FileHandle file) override;

private:
	std::string vfsModes;
};
#endif  // RMLUI_VFSFILEINTERFACE_H
