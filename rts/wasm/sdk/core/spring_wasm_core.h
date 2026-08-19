/* Spring Core-Wasm guest ABI. C11/C++ compatible. */
#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>

#if !defined(__wasm32__)
#error "spring_wasm_core.h is a wasm32 guest ABI header"
#endif

#if defined(__clang__)
#define SPRING_WASM_IMPORT(module_name, import_name) \
	__attribute__((import_module(module_name), import_name(import_name)))
#define SPRING_WASM_EXPORT(export_name) \
	__attribute__((export_name(export_name)))
#else
#error "The Core-Wasm C SDK currently requires Clang wasm attributes"
#endif

#ifdef __cplusplus
extern "C" {
#endif

enum SpringWasmStatus {
	SPRING_WASM_OK = 0,
	SPRING_WASM_INVALID_ARGUMENT = 1,
	SPRING_WASM_OUT_OF_BOUNDS = 2,
	SPRING_WASM_NOT_FOUND = 3,
	SPRING_WASM_NOT_AVAILABLE = 4,
	SPRING_WASM_INVALID_STATE = 5,
	SPRING_WASM_PERMISSION_DENIED = 6,
	SPRING_WASM_ALREADY_EXISTS = 7,
	SPRING_WASM_OPERATION_FAILED = 8,
	SPRING_WASM_BUFFER_OVERFLOW = 9,
	SPRING_WASM_INVALID_ID = 10,
	SPRING_WASM_INTERNAL = 999,
};

enum SpringWasmPositionFlags {
	SPRING_WASM_POSITION_MID = 1u << 0,
	SPRING_WASM_POSITION_AIM = 1u << 1,
};

typedef struct SpringWasmBufferResult {
	uint32_t count;
	int32_t status;
} SpringWasmBufferResult;

typedef struct SpringWasmUnitHealth {
	float health;
	float max_health;
	float paralyze_damage;
	float capture_progress;
	float build_progress;
} SpringWasmUnitHealth;

static inline int32_t spring_wasm_packed_i32(int64_t packed)
{
	return (int32_t)(uint32_t)(uint64_t)packed;
}

static inline int32_t spring_wasm_packed_status(int64_t packed)
{
	return (int32_t)(uint32_t)((uint64_t)packed >> 32);
}

static inline float spring_wasm_packed_f32(int64_t packed)
{
	const uint32_t bits = (uint32_t)(uint64_t)packed;
	float value;
	memcpy(&value, &bits, sizeof(value));
	return value;
}

static inline SpringWasmBufferResult spring_wasm_buffer_result(int64_t packed)
{
	SpringWasmBufferResult result;
	result.count = (uint32_t)(uint64_t)packed;
	result.status = spring_wasm_packed_status(packed);
	return result;
}

/* UnitsInfo */
SPRING_WASM_IMPORT("spring:units-info", "get-unit-def-id")
int64_t spring_get_unit_def_id_raw(int32_t unit_id);
SPRING_WASM_IMPORT("spring:units-info", "get-unit-team")
int64_t spring_get_unit_team_raw(int32_t unit_id);
SPRING_WASM_IMPORT("spring:units-info", "get-unit-is-dead")
int64_t spring_get_unit_is_dead_raw(int32_t unit_id);
SPRING_WASM_IMPORT("spring:units-info", "get-unit-experience")
int64_t spring_get_unit_experience_raw(int32_t unit_id);
SPRING_WASM_IMPORT("spring:units-info", "get-unit-position")
int32_t spring_get_unit_position_raw(int32_t unit_id, int32_t flags, int32_t output_ptr);
SPRING_WASM_IMPORT("spring:units-info", "get-unit-velocity")
int32_t spring_get_unit_velocity_raw(int32_t unit_id, int32_t output_ptr);
SPRING_WASM_IMPORT("spring:units-info", "get-unit-health")
int32_t spring_get_unit_health_raw(int32_t unit_id, int32_t output_ptr);

/* UnitsQuery: list capacities are element counts, not byte counts. */
SPRING_WASM_IMPORT("spring:units-query", "valid-unit-id")
int64_t spring_valid_unit_id_raw(int32_t unit_id);
SPRING_WASM_IMPORT("spring:units-query", "get-all-units")
int64_t spring_get_all_units_raw(int32_t output_ptr, int32_t capacity);
SPRING_WASM_IMPORT("spring:units-query", "get-team-units")
int64_t spring_get_team_units_raw(int32_t team_id, int32_t output_ptr, int32_t capacity);
SPRING_WASM_IMPORT("spring:units-query", "get-team-unit-def-count")
int64_t spring_get_team_unit_def_count_raw(int32_t team_id, int32_t unit_def_id);
SPRING_WASM_IMPORT("spring:units-query", "get-team-unit-count")
int64_t spring_get_team_unit_count_raw(int32_t team_id);
SPRING_WASM_IMPORT("spring:units-query", "get-units-in-rectangle")
int64_t spring_get_units_in_rectangle_raw(float xmin, float zmin, float xmax, float zmax,
	int32_t allegiance, int32_t output_ptr, int32_t capacity);
SPRING_WASM_IMPORT("spring:units-query", "get-units-in-box")
int64_t spring_get_units_in_box_raw(float xmin, float ymin, float zmin,
	float xmax, float ymax, float zmax, int32_t allegiance,
	int32_t output_ptr, int32_t capacity);
SPRING_WASM_IMPORT("spring:units-query", "get-units-in-sphere")
int64_t spring_get_units_in_sphere_raw(float x, float y, float z, float radius,
	int32_t allegiance, int32_t output_ptr, int32_t capacity);
SPRING_WASM_IMPORT("spring:units-query", "get-units-in-cylinder")
int64_t spring_get_units_in_cylinder_raw(float x, float z, float radius,
	int32_t allegiance, int32_t output_ptr, int32_t capacity);
SPRING_WASM_IMPORT("spring:units-query", "get-unit-nearest-ally")
int64_t spring_get_unit_nearest_ally_raw(int32_t unit_id, float range);
SPRING_WASM_IMPORT("spring:units-query", "get-unit-nearest-enemy")
int64_t spring_get_unit_nearest_enemy_raw(int32_t unit_id, float range, int32_t flags);
SPRING_WASM_IMPORT("spring:units-query", "get-unit-separation")
int64_t spring_get_unit_separation_raw(int32_t unit_id1, int32_t unit_id2, int32_t flags);

/* UnitDefs: buffers are raw bytes, no trailing NUL is written. */
SPRING_WASM_IMPORT("spring:unit-defs", "get-unit-def-name")
int64_t spring_get_unit_def_name_raw(int32_t unit_def_id, int32_t output_ptr, int32_t capacity_bytes);
SPRING_WASM_IMPORT("spring:unit-defs", "get-unit-def-human-name")
int64_t spring_get_unit_def_human_name_raw(int32_t unit_def_id, int32_t output_ptr, int32_t capacity_bytes);

static inline int spring_get_unit_def_id(int32_t unit_id, int32_t* value)
{
	const int64_t packed = spring_get_unit_def_id_raw(unit_id);
	const int32_t status = spring_wasm_packed_status(packed);
	if (status == 0 && value != NULL)
		*value = spring_wasm_packed_i32(packed);
	return status;
}

static inline int spring_get_unit_position(int32_t unit_id, uint32_t flags, float position[3])
{
	return spring_get_unit_position_raw(unit_id, (int32_t)flags,
		(int32_t)(uint32_t)(uintptr_t)position);
}

static inline SpringWasmBufferResult spring_get_team_units(
	int32_t team_id, int32_t* units, uint32_t capacity)
{
	return spring_wasm_buffer_result(spring_get_team_units_raw(team_id,
		(int32_t)(uint32_t)(uintptr_t)units, (int32_t)capacity));
}

/* Callin signatures. Guests export whichever callins they implement. */
typedef void (*spring_game_frame_fn)(int32_t frame);
typedef void (*spring_game_frame_post_fn)(int32_t frame);
typedef void (*spring_update_fn)(float delta_seconds);
typedef void (*spring_unit_created_fn)(int32_t unit_id, int32_t unit_def_id,
	int32_t unit_team, int32_t builder_id);
typedef int64_t (*spring_unit_pre_damaged_fn)(int32_t unit_id, int32_t unit_def_id,
	int32_t unit_team, float damage, int32_t paralyzer, int32_t weapon_def_id,
	int32_t projectile_id, int32_t attacker_id, int32_t attacker_def_id,
	int32_t attacker_team);
typedef int32_t (*spring_allow_unit_creation_fn)(int32_t unit_def_id, int32_t builder_id,
	int32_t builder_team, int32_t has_build_info, float x, float y, float z,
	int32_t facing);
typedef void (*spring_draw_world_fn)(void);

#ifdef __cplusplus
} /* extern "C" */
#endif
