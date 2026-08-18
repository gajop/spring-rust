/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

// Standalone floor measurement for the Wasmtime C API host boundary.  It times
// the two transports the engine can reach from C++ (core-Wasm imports and
// dynamically typed Component Model imports) in both directions, so the cost of
// the boundary can be compared against the typed Rust API harness next to it.

#include <wasmtime.h>
#include <wasmtime/component.h>

#include <algorithm>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <ctime>
#include <string>
#include <vector>

namespace {

double NowNs()
{
	timespec ts{};
	clock_gettime(CLOCK_MONOTONIC, &ts);
	return static_cast<double>(ts.tv_sec) * 1e9 + static_cast<double>(ts.tv_nsec);
}

[[noreturn]] void Die(const char* what, wasmtime_error_t* error, wasm_trap_t* trap)
{
	std::fprintf(stderr, "%s", what);
	if (error != nullptr) {
		wasm_byte_vec_t message;
		wasmtime_error_message(error, &message);
		std::fprintf(stderr, ": %.*s", static_cast<int>(message.size), message.data);
		wasm_byte_vec_delete(&message);
		wasmtime_error_delete(error);
	}
	if (trap != nullptr) {
		wasm_byte_vec_t message;
		wasm_trap_message(trap, &message);
		std::fprintf(stderr, ": %.*s", static_cast<int>(message.size), message.data);
		wasm_byte_vec_delete(&message);
		wasm_trap_delete(trap);
	}
	std::fprintf(stderr, "\n");
	std::exit(1);
}

void Check(const char* what, wasmtime_error_t* error, wasm_trap_t* trap = nullptr)
{
	if (error != nullptr || trap != nullptr)
		Die(what, error, trap);
}

std::vector<uint8_t> ReadFile(const std::string& path)
{
	std::FILE* file = std::fopen(path.c_str(), "rb");
	if (file == nullptr) {
		std::fprintf(stderr, "cannot open %s\n", path.c_str());
		std::exit(1);
	}
	std::fseek(file, 0, SEEK_END);
	const long size = std::ftell(file);
	std::fseek(file, 0, SEEK_SET);
	std::vector<uint8_t> bytes(static_cast<size_t>(size));
	if (std::fread(bytes.data(), 1, bytes.size(), file) != bytes.size()) {
		std::fprintf(stderr, "cannot read %s\n", path.c_str());
		std::exit(1);
	}
	std::fclose(file);
	return bytes;
}

// Mirrors the engine's WasmRuntime configuration so the floor is measured under
// the same Cranelift and trap settings the engine actually runs with.
wasm_engine_t* MakeEngine()
{
	wasm_config_t* config = wasm_config_new();
	wasmtime_config_consume_fuel_set(config, false);
	wasmtime_config_wasm_component_model_set(config, true);
	wasmtime_config_signals_based_traps_set(config, true);
	wasmtime_config_wasm_threads_set(config, false);
	wasmtime_config_shared_memory_set(config, false);
	wasmtime_config_wasm_relaxed_simd_set(config, false);
	wasmtime_config_wasm_relaxed_simd_deterministic_set(config, true);
	wasmtime_config_cranelift_nan_canonicalization_set(config, true);
	wasmtime_config_wasm_multi_value_set(config, true);
	wasmtime_config_wasm_bulk_memory_set(config, true);
	return wasm_engine_new_with_config(config);
}

struct Row {
	std::string name;
	double nsPerCall;
	double spreadNs;
};

std::vector<Row> rows;

void Report(const std::string& name, std::vector<double> samples, int64_t callsPerSample)
{
	std::sort(samples.begin(), samples.end());
	const double median = samples[samples.size() / 2] / static_cast<double>(callsPerSample);
	const double low = samples.front() / static_cast<double>(callsPerSample);
	const double high = samples.back() / static_cast<double>(callsPerSample);
	rows.push_back({name, median, high - low});
	std::printf("%-34s %9.2f ns  [%.2f .. %.2f]\n", name.c_str(), median, low, high);
	std::fflush(stdout);
}

// ---------------------------------------------------------------- core wasm

wasm_trap_t* CoreAddChecked(void*, wasmtime_caller_t*, const wasmtime_val_t* args, size_t,
	wasmtime_val_t* results, size_t)
{
	results[0].kind = WASMTIME_I32;
	results[0].of.i32 = args[0].of.i32 + 1;
	return nullptr;
}

wasm_trap_t* CoreAddUnchecked(void*, wasmtime_caller_t*, wasmtime_val_raw_t* argsAndResults,
	size_t)
{
	argsAndResults[0].i32 = argsAndResults[0].i32 + 1;
	return nullptr;
}

struct CoreHarness {
	wasmtime_store_t* store = nullptr;
	wasmtime_context_t* context = nullptr;
	wasmtime_instance_t instance{};

	wasmtime_func_t Lookup(const char* name) const
	{
		wasmtime_extern_t item;
		if (!wasmtime_instance_export_get(context, &instance, name, std::strlen(name), &item)) {
			std::fprintf(stderr, "core export %s is missing\n", name);
			std::exit(1);
		}
		return item.of.func;
	}
};

CoreHarness MakeCore(wasm_engine_t* engine, const std::vector<uint8_t>& bytes, bool unchecked)
{
	wasmtime_module_t* module = nullptr;
	Check("core compile", wasmtime_module_new(engine, bytes.data(), bytes.size(), &module));

	wasmtime_linker_t* linker = wasmtime_linker_new(engine);
	wasm_functype_t* type = wasm_functype_new_1_1(wasm_valtype_new_i32(), wasm_valtype_new_i32());
	if (unchecked) {
		Check("core define unchecked", wasmtime_linker_define_func_unchecked(linker, "spring", 6,
			"add_i32", 7, type, CoreAddUnchecked, nullptr, nullptr));
	} else {
		Check("core define", wasmtime_linker_define_func(linker, "spring", 6, "add_i32", 7, type,
			CoreAddChecked, nullptr, nullptr));
	}
	wasm_functype_delete(type);

	CoreHarness harness;
	harness.store = wasmtime_store_new(engine, nullptr, nullptr);
	harness.context = wasmtime_store_context(harness.store);
	wasm_trap_t* trap = nullptr;
	Check("core instantiate", wasmtime_linker_instantiate(linker, harness.context, module,
		&harness.instance, &trap), trap);
	return harness;
}

double CoreDrive(const CoreHarness& harness, const char* name, int32_t iterations)
{
	const wasmtime_func_t func = harness.Lookup(name);
	wasmtime_val_t args[1];
	args[0].kind = WASMTIME_I32;
	args[0].of.i32 = iterations;
	wasmtime_val_t results[1];
	wasm_trap_t* trap = nullptr;
	const double start = NowNs();
	Check("core call", wasmtime_func_call(harness.context, &func, args, 1, results, 1, &trap),
		trap);
	const double elapsed = NowNs() - start;
	if (results[0].of.i32 != iterations) {
		std::fprintf(stderr, "core %s returned %d, expected %d\n", name, results[0].of.i32,
			iterations);
		std::exit(1);
	}
	return elapsed;
}

// ---------------------------------------------------------------- component

wasmtime_error_t* ComponentAdd(void*, wasmtime_context_t*,
	const wasmtime_component_func_type_t*, wasmtime_component_val_t* args, size_t,
	wasmtime_component_val_t* results, size_t)
{
	results[0].kind = WASMTIME_COMPONENT_S32;
	results[0].of.s32 = args[0].of.s32 + 1;
	return nullptr;
}

// The shape the engine's generated adapters actually return: every callout is
// wrapped in result<T, spring-error>, so the ok payload is heap allocated.
wasmtime_error_t* ComponentAddResult(void*, wasmtime_context_t*,
	const wasmtime_component_func_type_t*, wasmtime_component_val_t* args, size_t,
	wasmtime_component_val_t* results, size_t)
{
	wasmtime_component_val_t payload;
	payload.kind = WASMTIME_COMPONENT_S32;
	payload.of.s32 = args[0].of.s32 + 1;
	results[0].kind = WASMTIME_COMPONENT_RESULT;
	results[0].of.result.is_ok = true;
	results[0].of.result.val = wasmtime_component_val_new(&payload);
	return nullptr;
}

// A record inside result<> is what most of the engine's callouts return.  The
// dynamic API owns whatever the callback hands back, so the record vector and
// every field name have to be freshly allocated on each call.
wasmtime_error_t* ComponentGetVec3(void*, wasmtime_context_t*,
	const wasmtime_component_func_type_t*, wasmtime_component_val_t* args, size_t,
	wasmtime_component_val_t* results, size_t)
{
	static const char* const fieldNames[3] = {"x", "y", "z"};
	const float base = static_cast<float>(args[0].of.s32 + 1);
	wasmtime_component_val_t vector;
	vector.kind = WASMTIME_COMPONENT_RECORD;
	wasmtime_component_valrecord_new_uninit(&vector.of.record, 3);
	for (int field = 0; field < 3; ++field) {
		wasm_name_new(&vector.of.record.data[field].name, 1, fieldNames[field]);
		vector.of.record.data[field].val.kind = WASMTIME_COMPONENT_F32;
		vector.of.record.data[field].val.of.f32 = base;
	}
	results[0].kind = WASMTIME_COMPONENT_RESULT;
	results[0].of.result.is_ok = true;
	results[0].of.result.val = wasmtime_component_val_new(&vector);
	return nullptr;
}

// Same result shape as ComponentGetVec3, but with the incoming record argument
// the engine's get-unit-position actually takes.  Wasmtime builds that argument
// as a val tree before the callback runs, whichever way the callback is written.
wasmtime_error_t* ComponentGetVec3Opts(void*, wasmtime_context_t*,
	const wasmtime_component_func_type_t*, wasmtime_component_val_t* args, size_t,
	wasmtime_component_val_t* results, size_t)
{
	static const char* const fieldNames[3] = {"x", "y", "z"};
	const float base = static_cast<float>(args[0].of.s32 + 1);
	wasmtime_component_val_t vector;
	vector.kind = WASMTIME_COMPONENT_RECORD;
	wasmtime_component_valrecord_new_uninit(&vector.of.record, 3);
	for (int field = 0; field < 3; ++field) {
		wasm_name_new(&vector.of.record.data[field].name, 1, fieldNames[field]);
		vector.of.record.data[field].val.kind = WASMTIME_COMPONENT_F32;
		vector.of.record.data[field].val.of.f32 = base;
	}
	results[0].kind = WASMTIME_COMPONENT_RESULT;
	results[0].of.result.is_ok = true;
	results[0].of.result.val = wasmtime_component_val_new(&vector);
	return nullptr;
}

struct ComponentHarness {
	wasmtime_store_t* store = nullptr;
	wasmtime_context_t* context = nullptr;
	wasmtime_component_instance_t instance{};

	wasmtime_component_func_t Lookup(const char* name) const
	{
		wasmtime_component_export_index_t* index =
			wasmtime_component_instance_get_export_index(&instance, context, nullptr, name,
				std::strlen(name));
		if (index == nullptr) {
			std::fprintf(stderr, "component export %s is missing\n", name);
			std::exit(1);
		}
		wasmtime_component_func_t func{};
		if (!wasmtime_component_instance_get_func(&instance, context, index, &func)) {
			std::fprintf(stderr, "component export %s is not a function\n", name);
			std::exit(1);
		}
		wasmtime_component_export_index_delete(index);
		return func;
	}
};

ComponentHarness MakeComponent(wasm_engine_t* engine, const std::vector<uint8_t>& bytes)
{
	wasmtime_component_t* component = nullptr;
	Check("component compile", wasmtime_component_new(engine, bytes.data(), bytes.size(),
		&component));

	wasmtime_component_linker_t* linker = wasmtime_component_linker_new(engine);
	wasmtime_component_linker_instance_t* root = wasmtime_component_linker_root(linker);
	const char* interfaceName = "recoil:hostcall-bench/host@1.0.0";
	wasmtime_component_linker_instance_t* hostInstance = nullptr;
	Check("component add instance", wasmtime_component_linker_instance_add_instance(root,
		interfaceName, std::strlen(interfaceName), &hostInstance));
	Check("component add add-i32", wasmtime_component_linker_instance_add_func(hostInstance,
		"add-i32", 7, ComponentAdd, nullptr, nullptr));
	Check("component add add-i32-result", wasmtime_component_linker_instance_add_func(hostInstance,
		"add-i32-result", 14, ComponentAddResult, nullptr, nullptr));
	Check("component add get-vec3", wasmtime_component_linker_instance_add_func(hostInstance,
		"get-vec3", 8, ComponentGetVec3, nullptr, nullptr));
	Check("component add get-vec3-opts", wasmtime_component_linker_instance_add_func(hostInstance,
		"get-vec3-opts", 13, ComponentGetVec3Opts, nullptr, nullptr));
	wasmtime_component_linker_instance_delete(hostInstance);
	wasmtime_component_linker_instance_delete(root);

	ComponentHarness harness;
	harness.store = wasmtime_store_new(engine, nullptr, nullptr);
	harness.context = wasmtime_store_context(harness.store);
	Check("component instantiate", wasmtime_component_linker_instantiate(linker, harness.context,
		component, &harness.instance));
	return harness;
}

double ComponentDrive(const ComponentHarness& harness, const char* name, int32_t iterations)
{
	const wasmtime_component_func_t func = harness.Lookup(name);
	wasmtime_component_val_t args[1];
	args[0].kind = WASMTIME_COMPONENT_S32;
	args[0].of.s32 = iterations;
	wasmtime_component_val_t results[1];
	std::memset(results, 0, sizeof(results));
	const double start = NowNs();
	Check("component call", wasmtime_component_func_call(&func, harness.context, args, 1, results,
		1));
	const double elapsed = NowNs() - start;
	if (results[0].of.s32 != iterations) {
		std::fprintf(stderr, "component %s returned %d, expected %d\n", name, results[0].of.s32,
			iterations);
		std::exit(1);
	}
	wasmtime_component_val_delete(&results[0]);
	return elapsed;
}

} // namespace

int main(int argc, char** argv)
{
	const std::string root = argc > 1 ? argv[1] : ".";
	const int32_t iterations = argc > 2 ? std::atoi(argv[2]) : 200000;
	const int repeats = argc > 3 ? std::atoi(argv[3]) : 21;

	const std::vector<uint8_t> coreBytes = ReadFile(root + "/core.wasm");
	const std::vector<uint8_t> componentBytes = ReadFile(root + "/bench_component.wasm");
	wasm_engine_t* engine = MakeEngine();

	std::printf("wasmtime C API, %d guest iterations per sample, %d samples\n\n", iterations,
		repeats);

	const CoreHarness coreChecked = MakeCore(engine, coreBytes, false);
	const CoreHarness coreUnchecked = MakeCore(engine, coreBytes, true);
	const ComponentHarness component = MakeComponent(engine, componentBytes);

	// Warm up every path so the first timed sample is not paying for lazy
	// trampoline compilation or cold instruction cache.
	for (int warm = 0; warm < 3; ++warm) {
		CoreDrive(coreChecked, "run_callout", iterations);
		CoreDrive(coreUnchecked, "run_callout", iterations);
		CoreDrive(coreChecked, "run_spin", iterations);
		ComponentDrive(component, "run-callout", iterations);
		ComponentDrive(component, "run-callout-result", iterations);
		ComponentDrive(component, "run-callout-vec3", iterations);
		ComponentDrive(component, "run-callout-vec3-opts", iterations);
		ComponentDrive(component, "run-spin", iterations);
	}

	std::vector<double> spinCore, spinComponent, calloutCoreChecked, calloutCoreUnchecked,
		calloutComponent, calloutComponentResult, calloutComponentVec3, calloutComponentVec3Opts;
	for (int sample = 0; sample < repeats; ++sample) {
		spinCore.push_back(CoreDrive(coreChecked, "run_spin", iterations));
		spinComponent.push_back(ComponentDrive(component, "run-spin", iterations));
		calloutCoreChecked.push_back(CoreDrive(coreChecked, "run_callout", iterations));
		calloutCoreUnchecked.push_back(CoreDrive(coreUnchecked, "run_callout", iterations));
		calloutComponent.push_back(ComponentDrive(component, "run-callout", iterations));
		calloutComponentResult.push_back(ComponentDrive(component, "run-callout-result",
			iterations));
		calloutComponentVec3.push_back(ComponentDrive(component, "run-callout-vec3", iterations));
		calloutComponentVec3Opts.push_back(ComponentDrive(component, "run-callout-vec3-opts",
			iterations));
	}

	std::printf("guest -> host (callout), per call\n");
	Report("core, C API checked", calloutCoreChecked, iterations);
	Report("core, C API unchecked", calloutCoreUnchecked, iterations);
	Report("component, C API dynamic", calloutComponent, iterations);
	Report("component, C API dynamic result<>", calloutComponentResult, iterations);
	Report("component, C API dynamic vec3", calloutComponentVec3, iterations);
	Report("component, C API dyn vec3+record arg", calloutComponentVec3Opts, iterations);
	Report("guest loop only (core)", spinCore, iterations);
	Report("guest loop only (component)", spinComponent, iterations);

	// host -> guest, one component/core entry per call
	const wasmtime_func_t coreNoop = coreChecked.Lookup("noop");
	const wasmtime_component_func_t componentNoop = component.Lookup("noop");

	const wasmtime_component_func_t componentNoopRecord = component.Lookup("noop-record");

	std::vector<double> callinCore, callinCoreUnchecked, callinComponent, callinComponentRecord;
	const int32_t callinIterations = iterations / 20;
	for (int sample = 0; sample < repeats; ++sample) {
		{
			wasmtime_val_t args[1];
			args[0].kind = WASMTIME_I32;
			args[0].of.i32 = 7;
			wasmtime_val_t results[1];
			wasm_trap_t* trap = nullptr;
			const double start = NowNs();
			for (int32_t call = 0; call < callinIterations; ++call) {
				Check("core callin", wasmtime_func_call(coreChecked.context, &coreNoop, args, 1,
					results, 1, &trap), trap);
			}
			callinCore.push_back(NowNs() - start);
		}
		{
			wasmtime_val_raw_t slot[1];
			slot[0].i32 = 7;
			wasm_trap_t* trap = nullptr;
			const double start = NowNs();
			for (int32_t call = 0; call < callinIterations; ++call) {
				Check("core callin unchecked", wasmtime_func_call_unchecked(coreChecked.context,
					&coreNoop, slot, 1, &trap), trap);
			}
			callinCoreUnchecked.push_back(NowNs() - start);
		}
		{
			wasmtime_component_val_t args[1];
			args[0].kind = WASMTIME_COMPONENT_S32;
			args[0].of.s32 = 7;
			wasmtime_component_val_t results[1];
			std::memset(results, 0, sizeof(results));
			const double start = NowNs();
			for (int32_t call = 0; call < callinIterations; ++call) {
				Check("component callin", wasmtime_component_func_call(&componentNoop,
					component.context, args, 1, results, 1));
				wasmtime_component_val_delete(&results[0]);
				std::memset(results, 0, sizeof(results));
			}
			callinComponent.push_back(NowNs() - start);
		}
		{
			// The engine's generated callins pass records, and the dynamic C API
			// requires one named entry per field.  The entries are built once and
			// reused, which is the best case the current adapter can reach.
			wasmtime_component_valrecord_entry_t entries[4];
			static const char* const fieldNames[4] = {"a", "b", "c", "d"};
			for (int field = 0; field < 4; ++field) {
				entries[field].name.data = const_cast<char*>(fieldNames[field]);
				entries[field].name.size = 1;
				entries[field].val.kind = WASMTIME_COMPONENT_S32;
				entries[field].val.of.s32 = 7 + field;
			}
			wasmtime_component_val_t args[1];
			args[0].kind = WASMTIME_COMPONENT_RECORD;
			args[0].of.record.size = 4;
			args[0].of.record.data = entries;
			wasmtime_component_val_t results[1];
			std::memset(results, 0, sizeof(results));
			const double start = NowNs();
			for (int32_t call = 0; call < callinIterations; ++call) {
				Check("component record callin", wasmtime_component_func_call(
					&componentNoopRecord, component.context, args, 1, results, 1));
				wasmtime_component_val_delete(&results[0]);
				std::memset(results, 0, sizeof(results));
			}
			callinComponentRecord.push_back(NowNs() - start);
		}
	}

	std::printf("\nhost -> guest (callin), per call\n");
	Report("core, C API checked", callinCore, callinIterations);
	Report("core, C API unchecked", callinCoreUnchecked, callinIterations);
	Report("component, C API dynamic", callinComponent, callinIterations);
	Report("component, C API dynamic record", callinComponentRecord, callinIterations);

	return 0;
}
