/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

// Alternate Wasm hosts used by the benchmark suite. The Rust typed Component
// host remains dlopen'd, while the C++ Core host is built into the engine. The
// public static seam multiplexes both so NativeInterfaceEventClient needs only
// one fast-path hook.

#include <cstddef>
#include <cstdint>
#include <string>
#include <string_view>
#include <vector>

struct NativeInterface;

// Mirrors `SpringTypedCommand` in rust/crates/spring-wasm-typed-host/src/ffi.rs.
struct SpringTypedCommand {
	std::int32_t cmdID;
	std::int32_t tag;
	std::int32_t aiCommandID;
	float timeOut;
	std::uint32_t paramOffset;
	std::uint32_t paramCount;
	std::uint8_t options;
};

// Mirrors `ShimTable` in the same file. Field order is load-bearing.
struct SpringTypedShimTable {
	std::int32_t (*messages_send_lua_rules_msg)(void*, const char*, std::size_t, bool*);
	std::int32_t (*profiling_get_timer_micros)(void*, std::uint8_t, std::uint64_t*);
	std::int32_t (*rules_params_get_unit_rules_param)(void*, std::int32_t, const char*,
		std::size_t, std::int32_t*, bool*, float*, const char**, std::size_t*,
		std::int32_t*, bool*);
	std::int32_t (*rules_params_set_unit_rules_param)(void*, std::int32_t, const char*,
		std::size_t, std::int32_t, bool, float, const char*, std::size_t, std::int32_t,
		bool*);
	std::int32_t (*terrain_get_ground_orig_height)(void*, float, float, float*);
	std::int32_t (*terrain_control_level_height_map)(void*, float, float, float, float,
		float, bool*);
	std::int32_t (*terrain_control_set_height_map)(void*, float, float, float, float, bool*);
	std::int32_t (*unit_control_give_order_to_unit)(void*, std::int32_t, std::int32_t,
		const float*, std::size_t, std::uint32_t, std::int32_t, bool*);
	std::int32_t (*unit_defs_get_unit_def_name)(void*, std::int32_t, const char**, std::size_t*);
	std::int32_t (*units_commands_get_unit_commands)(void*, std::int32_t, std::uint32_t,
		const SpringTypedCommand**, std::size_t*, const float**, std::size_t*);
	std::int32_t (*units_info_get_unit_def_id)(void*, std::int32_t, std::int32_t*);
	std::int32_t (*units_info_get_unit_health)(void*, std::int32_t, float*);
	std::int32_t (*units_info_get_unit_position)(void*, std::int32_t, bool, bool, float*);
	std::int32_t (*units_query_get_team_units)(void*, std::int32_t, const std::int32_t**,
		std::size_t*);
	std::int32_t (*units_query_get_units_in_cylinder)(void*, float, float, float,
		std::int32_t, const std::int32_t**, std::size_t*);
	std::int32_t (*terrain_control_set_height_map_func)(void*, void (*)(void*), void*, bool*);
	std::int32_t (*profiling_get_lua_mem_usage)(void*, std::uint8_t, float*);
	std::int32_t (*profiling_get_synced_gc_info)(void*, bool, float*);
	std::int32_t (*messages_send_lua_ui_msg)(void*, const char*, std::size_t, const char*,
		std::size_t, bool*);
	std::int32_t (*gfx_vertex)(void*, float, float, float, float, std::uint32_t);
	std::int32_t (*gfx_begin_end)(void*, std::uint32_t, void (*)(void*), void*);
};

enum class SpringTypedWorld : std::uint8_t {
	RulesSynced = 0,
	RulesUnsynced = 1,
	UI = 2,
};

const SpringTypedShimTable& TypedHostShimTable();

struct SpringTypedHostLibrary {
	void* handle = nullptr;

	void* (*hostNew)(const std::uint8_t*, std::size_t, void*, const SpringTypedShimTable*,
		std::uint8_t, char**) = nullptr;
	void (*hostFree)(void*) = nullptr;
	void (*stringFree)(char*) = nullptr;
	std::int32_t (*callinGameFrame)(void*, std::int32_t, char**) = nullptr;
	std::int32_t (*callinGameFramePost)(void*, std::int32_t, char**) = nullptr;
	std::int32_t (*callinUpdate)(void*, float, char**) = nullptr;
	std::int32_t (*callinUnitCreated)(void*, std::int32_t, std::int32_t, std::int32_t,
		std::int32_t, char**) = nullptr;
	std::int32_t (*callinUnitPreDamaged)(void*, std::int32_t, std::int32_t, std::int32_t,
		float, bool, std::int32_t, std::int32_t, std::int32_t, std::int32_t, std::int32_t,
		float*, float*, char**) = nullptr;
	std::int32_t (*callinAllowUnitCreation)(void*, std::int32_t, std::int32_t, std::int32_t,
		bool, float, float, float, std::int32_t, bool*, bool*, char**) = nullptr;
	std::int32_t (*callinDrawWorld)(void*, char**) = nullptr;
};

class WasmTypedHost {
public:
	// True when either benchmark transport is selected. NativeInterfaceEventClient
	// uses this as the single alternate-host gate.
	static bool Enabled();
	// True only for the Rust typed Component host.
	static bool TypedEnabled();

	static bool Load(std::string moduleName,
		const std::vector<std::uint8_t>& componentBytes,
		NativeInterface* nativeInterface, SpringTypedWorld world, std::string& error);
	static void Unload(std::string_view moduleName);
	static void UnloadAll();
	static bool AnyActive();
	// `synced` comes from the NativeInterface event seam; it is forwarded to the
	// Core dispatch below, which needs the side rather than guessing it.
	static bool DispatchCallin(std::string_view name, const void* query, bool synced,
		void* result, std::string& error);

	~WasmTypedHost();

	WasmTypedHost(const WasmTypedHost&) = delete;
	WasmTypedHost& operator=(const WasmTypedHost&) = delete;

private:
	WasmTypedHost(std::string moduleName, void* host)
		: moduleName(std::move(moduleName)), host(host) {}

	enum class Callin : std::uint8_t {
		None,
		GameFrame,
		GameFramePost,
		Update,
		UnitCreated,
		UnitPreDamaged,
		AllowUnitCreation,
		DrawWorld,
	};
	static Callin Resolve(std::string_view name);

	bool Invoke(Callin callin, const void* query, void* result, std::string& error) const;

	std::string moduleName;
	void* host = nullptr;
};
