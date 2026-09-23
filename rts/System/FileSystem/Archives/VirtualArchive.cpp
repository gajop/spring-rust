/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "VirtualArchive.h"
#include "System/FileSystem/FileSystem.h"
#include "System/FileSystem/DataDirsAccess.h"
#include "System/FileSystem/FileQueryFlags.h"
#include "System/Log/ILog.h"
#include "System/StringUtil.h"

#include "zlib.h"
#include "minizip/zip.h"
#include <cassert>

CVirtualArchiveFactory* virtualArchiveFactory;

CVirtualArchiveFactory::CVirtualArchiveFactory() : IArchiveFactory("sva")
{
	virtualArchiveFactory = this;
}

CVirtualArchiveFactory::~CVirtualArchiveFactory()
{
	virtualArchiveFactory = nullptr;
}


static uint32_t NextVirtualArchiveGeneration()
{
	// Process-local; virtual archives never outlive the process.
	static uint32_t generation = 0;
	return ++generation;
}

CVirtualArchive* CVirtualArchiveFactory::AddArchive(const std::string& fileName)
{
	// Re-adding a name (e.g. regenerating a map) supersedes the older archive;
	// lookups search newest first. Older archives stay alive for open handles.
	CVirtualArchive* archive = new CVirtualArchive(fileName, NextVirtualArchiveGeneration());
	archives.push_back(archive);
	return archive;
}

static CVirtualArchive* FindNewest(const std::vector<CVirtualArchive*>& archives, const std::string& baseName)
{
	for (auto it = archives.rbegin(); it != archives.rend(); ++it) {
		if ((*it)->GetFileName() == baseName)
			return *it;
	}

	return nullptr;
}

uint32_t CVirtualArchiveFactory::GetGeneration(const std::string& baseName) const
{
	const CVirtualArchive* archive = FindNewest(archives, baseName);
	return (archive != nullptr)? archive->GetGeneration(): 0;
}

IArchive* CVirtualArchiveFactory::DoCreateArchive(const std::string& fileName) const
{
	CVirtualArchive* archive = FindNewest(archives, FileSystem::GetBasename(fileName));
	return (archive != nullptr)? archive->Open(): nullptr;
}

CVirtualArchiveOpen::CVirtualArchiveOpen(CVirtualArchive* archive, const std::string& fileName)
	: IArchive(fileName)
	, archive(archive)
{
	// set subclass name index to archive's index (doesn't update while archive is open)
	lcNameIndex = archive->GetNameIndex();
}


uint32_t CVirtualArchiveOpen::NumFiles() const
{
	return archive->NumFiles();
}

bool CVirtualArchiveOpen::GetFile(uint32_t fid, std::vector<std::uint8_t>& buffer)
{
	return archive->GetFile(fid, buffer);
}

const std::string& CVirtualArchiveOpen::FileName(uint32_t fid) const
{
	return archive->FileName(fid);
}

int32_t CVirtualArchiveOpen::FileSize(uint32_t fid) const
{
	return archive->FileSize(fid);
}

IArchive::SFileInfo CVirtualArchiveOpen::FileInfo(uint32_t fid) const
{
	return archive->FileInfo(fid);
}



CVirtualArchiveOpen* CVirtualArchive::Open()
{
	return new CVirtualArchiveOpen(this, fileName);
}


bool CVirtualArchive::GetFile(uint32_t fid, std::vector<std::uint8_t>& buffer)
{
	if (fid >= files.size())
		return false;

	buffer = files[fid].buffer;
	return true;
}

const std::string& CVirtualArchive::FileName(uint32_t fid) const
{
	assert(fid < files.size());
	return files[fid].name;
}

int32_t CVirtualArchive::FileSize(uint32_t fid) const
{
	assert(fid < files.size());
	return static_cast<int32_t>(files[fid].buffer.size());
}

IArchive::SFileInfo CVirtualArchive::FileInfo(uint32_t fid) const
{
	assert(fid < files.size());
	const auto& fe = files[fid];
	return IArchive::SFileInfo{
		.fileName = fe.name,
		.specialFileName = "",
		.size = static_cast<int32_t>(fe.buffer.size()),
		// no file time in memory; the (nonzero) generation lets the archive
		// scanner treat unchanged content as cached
		.modTime = generation
	};
}

uint32_t CVirtualArchive::AddFile(const std::string& name)
{
	// Archive paths are case-insensitive throughout VFS.  Keep the index in
	// the same normalized form as IArchive::FindFile; generated virtual
	// archives may add paths such as LuaGaia/... after construction.
	lcNameIndex[StringToLower(name)] = files.size();
	files.emplace_back(files.size(), name);
	generation = NextVirtualArchiveGeneration();

	return (files.size() - 1);
}

void CVirtualArchive::WriteToFile()
{
	const std::string zipFilePath = dataDirsAccess.LocateFile(fileName, FileQueryFlags::WRITE) + ".sdz";
	LOG("Writing zip file for virtual archive %s to %s", fileName.c_str(), zipFilePath.c_str());

	zipFile zip = zipOpen(zipFilePath.c_str(), APPEND_STATUS_CREATE);

	if (zip == nullptr) {
		LOG("[VirtualArchive::%s] could not open zip file %s for writing", __func__, zipFilePath.c_str());
		return;
	}

	for (const CVirtualFile& file: files) {
		file.WriteZip(zip);
	}

	zipClose(zip, nullptr);
}

void CVirtualFile::WriteZip(void* zf) const
{
	zipFile zip = static_cast<zipFile>(zf);

	zipOpenNewFileInZip(zip, name.c_str(), nullptr, nullptr, 0, nullptr, 0, nullptr, Z_DEFLATED, Z_BEST_COMPRESSION);
	zipWriteInFileInZip(zip, buffer.data(), buffer.size());
	zipCloseFileInZip(zip);
}
