/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmSyncVerification.h"

#include <fstream>
#include <iomanip>
#include <sstream>
#include <stdexcept>

#include "WasmCoreHost.h"
#include "System/Config/ConfigHandler.h"
#include "System/Sync/SyncChecker.h"
#include "System/Threading/ThreadPool.h"
#include "Sim/Misc/ModInfo.h"
#include "Game/GlobalUnsynced.h"

CONFIG(std::string, WasmSyncLog).defaultValue("").description("Per-frame engine and synced Wasm state JSONL log; empty disables logging.");
CONFIG(int, WasmSyncDumpFrame).defaultValue(-1).description("Diagnostic frame at which to dump synced Wasm memories beside WasmSyncLog.");
CONFIG(int, WasmSyncStopFrame).defaultValue(-1).description("Exit after logging this simulation frame; -1 disables the diagnostic limit.");

namespace {

std::string JsonString(std::string_view value)
{
	std::ostringstream out;
	out << '"';
	for (unsigned char c: value) {
		if (c == '"' || c == '\\')
			out << '\\' << c;
		else if (c < 32)
			out << "\\u" << std::hex << std::setw(4) << std::setfill('0') << unsigned(c) << std::dec;
		else
			out << c;
	}
	out << '"';
	return out.str();
}

void Sync64(std::uint64_t value)
{
#ifdef SYNCCHECK
	CSyncChecker::SyncMeta(static_cast<std::uint32_t>(value));
	CSyncChecker::SyncMeta(static_cast<std::uint32_t>(value >> 32));
#endif
}

} // namespace

void WasmSyncVerification::EndFrame(int frame)
{
	static std::ofstream log;
	static std::string path;
	if (frame == 0) {
		log.close();
		log.clear();
		path = configHandler->GetString("WasmSyncLog");
		if (!path.empty()) {
			log.open(path, std::ios::trunc);
			if (!log)
				throw std::runtime_error("Cannot open WasmSyncLog: " + path);
		}
	}
	// Basic metadata stays inexpensive in ordinary play. A game can opt into
	// full-memory network checks; local diagnostic logging must NEVER change
	// the bytes mixed into that checksum, even when only one peer is logging.
#ifdef SYNCCHECK
	const bool mixMemory = modInfo.wasmStateHashInterval > 0 &&
		(frame % modInfo.wasmStateHashInterval) == 0;
#else
	const bool mixMemory = false;
#endif
	const bool dump = log.is_open() && frame == configHandler->GetInt("WasmSyncDumpFrame");
#ifndef SYNCCHECK
	if (!log.is_open() && !dump)
		return;
#endif
	const auto states = WasmCoreHost::SyncedStates(mixMemory || log.is_open() || dump);
#ifdef SYNCCHECK
	const unsigned engineChecksum = CSyncChecker::GetChecksum();
	const unsigned simChecksum = CSyncChecker::GetEngineChecksum();
	if (!states.empty()) {
		CSyncChecker::SyncMeta(0x5741534du);
		CSyncChecker::SyncMeta(static_cast<std::uint32_t>(states.size()));
		for (const auto& state: states) {
			CSyncChecker::SyncMeta(state.name.data(), state.name.size());
			CSyncChecker::SyncMeta(state.identity.data(), state.identity.size());
			CSyncChecker::SyncMeta(static_cast<std::uint32_t>(state.environment));
			CSyncChecker::SyncMeta(static_cast<std::uint32_t>(state.faulted));
			Sync64(state.memoryBytes);
			if (mixMemory) {
				Sync64(state.memoryHash);
				Sync64(state.stateHash);
			}
			Sync64(state.jobsHash);
		}
	}
	const unsigned checksum = CSyncChecker::GetChecksum();
#else
	const unsigned engineChecksum = 0;
	const unsigned simChecksum = 0;
	const unsigned checksum = 0;
#endif
	if (log.is_open()) {
		log << "{\"schema\":2,\"frame\":" << frame << ",\"engine\":" << engineChecksum
			<< ",\"sim\":" << simChecksum
			<< ",\"workers\":" << ThreadPool::GetNumThreads()
			<< ",\"checksum\":" << checksum << ",\"synccheck\":"
#ifdef SYNCCHECK
			<< "true"
#else
			<< "false"
#endif
			<< ",\"modules\":[";
		for (std::size_t i = 0; i < states.size(); ++i) {
			const auto& state = states[i];
			if (i != 0) log << ',';
			log << "{\"name\":" << JsonString(state.name)
				<< ",\"identity\":" << JsonString(state.identity)
				<< ",\"environment\":" << JsonString(WasmEnvironmentMatrix::Name(state.environment))
				<< ",\"bytes\":" << state.memoryBytes << ",\"hash\":\""
				<< std::hex << std::setw(16) << std::setfill('0') << state.memoryHash << std::dec
				<< "\",\"state_bytes\":" << state.stateBytes
				<< ",\"state_hash\":\"" << std::hex << std::setw(16) << state.stateHash << std::dec
				<< "\",\"jobs_hash\":\"" << std::hex << std::setw(16) << state.jobsHash << std::dec
				<< "\",\"faulted\":" << (state.faulted ? "true" : "false") << '}';
			if (dump && !WasmCoreHost::DumpSyncedMemory(state.name,
					path + ".frame-" + std::to_string(frame) + ".module-" + std::to_string(i) + ".bin"))
				throw std::runtime_error("Cannot dump synced Wasm memory: " + state.name);
			if (dump && !WasmCoreHost::DumpSyncedState(state.name,
					path + ".frame-" + std::to_string(frame) + ".module-" + std::to_string(i) + ".state.bin"))
				throw std::runtime_error("Cannot dump synced Wasm globals/tables: " + state.name);
		}
		log << "]}\n";
		log.flush();
		if (!log)
			throw std::runtime_error("Cannot write WasmSyncLog: " + path);
	}
	if (frame == configHandler->GetInt("WasmSyncStopFrame"))
		gu->globalQuit = true;
}
