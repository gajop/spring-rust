/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <stdint.h>

#include "NativeInterface/api/Common.h"

#ifdef __cplusplus
extern "C" {
#endif

// Immediate operations exposed by a CUS UnitEngine.  The numeric values
// are part of the native/Core-Wasm wire contract and must remain append-only.
enum CusOperation {
	CUS_OP_TURN = 0,
	CUS_OP_MOVE,
	CUS_OP_SPIN,
	CUS_OP_STOP_SPIN,
	CUS_OP_SCALE,
	CUS_OP_MOVE_NOW,
	CUS_OP_TURN_NOW,
	CUS_OP_SCALE_NOW,
	CUS_OP_SHOW,
	CUS_OP_HIDE,
	CUS_OP_EXPLODE,
	CUS_OP_EMIT_SFX,
	CUS_OP_ATTACH_UNIT,
	CUS_OP_DROP_UNIT,
	CUS_OP_SET_UNIT_VALUE,
	CUS_OP_AIM_SCRIPT_FINISHED,
	CUS_OP_AIM_SHIELD_FINISHED,
	CUS_OP_KILLED_SCRIPT_FINISHED,
};

struct CusAttachQuery {
	int32_t unitID;
	uint32_t instanceID;
	uint64_t capabilities;
};

struct CusAttachResult {
	const struct Error* error;
	uint8_t attached;
};

// `first`, `second`, and `third` carry operation-specific floating-point
// values.  `piece`, `axis`, `target`, and `value` carry operation-specific
// integer values.  The adapter validates the unit and instance before using
// any of them.
struct CusOperationQuery {
	int32_t unitID;
	uint32_t instanceID;
	uint32_t operation;
	int32_t piece;
	int32_t axis;
	int32_t target;
	int32_t value;
	float first;
	float second;
	float third;
};

struct CusOperationResult {
	const struct Error* error;
	int32_t value;
	uint8_t completed;
};

struct CusAnimationQuery {
	int32_t unitID;
	uint32_t instanceID;
	uint32_t animation;
	int32_t piece;
	int32_t axis;
};

struct CusAnimationResult {
	const struct Error* error;
	uint8_t active;
};

struct CusApi {
	void (*Attach)(const CusAttachQuery* query, CusAttachResult* result);
	void (*Operation)(const CusOperationQuery* query, CusOperationResult* result);
	void (*AnimationActive)(const CusAnimationQuery* query, CusAnimationResult* result);
};

// Optional module exports.  They deliberately use the same Query/Result
// convention as the ordinary native callins, so a module which does not use
// CUS remains ABI-compatible and does not need to export any of them.
struct CusInvokeQuery {
	uint32_t instanceID;
	uint32_t call;
	const float* floatArguments;
	uint32_t floatCount;
	const int32_t* integerArguments;
	uint32_t integerCount;
};

struct CusInvokeResult {
	const struct Error* error;
	uint8_t handled;
	int32_t intValue;
	float floatValue;
	uint8_t boolValue;
	uint8_t complete;
	int32_t* intValues;
	uint32_t intCapacity;
	uint32_t intCount;
	float* returnValues;
	uint32_t returnCapacity;
	uint32_t returnCount;
	uint8_t functionFound;
};

struct CusNamedQuery {
	uint32_t instanceID;
	const char* functionName;
	const float* arguments;
	uint32_t argumentCount;
	float* returnValues;
	uint32_t returnCapacity;
};

struct CusNamedResult {
	const struct Error* error;
	uint8_t handled;
	uint8_t functionFound;
	uint32_t returnCount;
};

struct CusTickQuery {
	uint32_t frame;
};

struct CusTickResult {
	const struct Error* error;
};

struct CusDetachQuery {
	uint32_t instanceID;
};

struct CusDetachResult {
	const struct Error* error;
};

#ifdef __cplusplus
}
#endif
