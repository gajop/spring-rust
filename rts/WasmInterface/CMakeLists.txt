include(${CMAKE_CURRENT_SOURCE_DIR}/WasmtimeDependency.cmake)
recoil_wasm_configure_wasmtime(RECOIL_WASMTIME_TARGET)

set(sources_engine_WasmInterface
	${CMAKE_CURRENT_SOURCE_DIR}/WasmEnvironment.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmHost.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreAbi.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreUnitsQueryBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreUnitDefsBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreUnitsCommandsBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreUnitControlBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreTerrainControlBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreGfxBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreProfilingBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreBenchmarkBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreHost.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreValidation.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmModule.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmModuleManifest.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmDispatch.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmResources.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmRuntime.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmTypedHost.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmTypedHostShims.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmInterfaceSystem.cpp
)

# Core API codegen writes these translation units into the checked-in generated
# tree. Keep the engine build usable before the first regeneration, then pick
# up the generated fixed/option/variable surfaces automatically.
foreach(RECOIL_WASM_CORE_GENERATED_SOURCE
	${CMAKE_CURRENT_SOURCE_DIR}/../wasm/generated/WasmCoreGeneratedBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/../wasm/generated/WasmCoreGeneratedOptionBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/../wasm/generated/WasmCoreGeneratedVariableBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/../wasm/generated/WasmCoreGeneratedVariableOutputBindings.cpp
)
	if(EXISTS ${RECOIL_WASM_CORE_GENERATED_SOURCE})
		list(APPEND sources_engine_WasmInterface ${RECOIL_WASM_CORE_GENERATED_SOURCE})
	endif()
endforeach()

set(sources_engine_WasmInterface ${sources_engine_WasmInterface} PARENT_SCOPE)

list(APPEND engineCommonLibraries ${RECOIL_WASMTIME_TARGET})
set(engineCommonLibraries "${engineCommonLibraries}" PARENT_SCOPE)
list(APPEND engineIncludes "${CMAKE_CURRENT_SOURCE_DIR}/../wasm")
set(engineIncludes "${engineIncludes}" PARENT_SCOPE)
set(RECOIL_WASMTIME_TARGET "${RECOIL_WASMTIME_TARGET}" PARENT_SCOPE)