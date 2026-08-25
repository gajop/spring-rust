include(${CMAKE_CURRENT_SOURCE_DIR}/WasmtimeDependency.cmake)
recoil_wasm_configure_wasmtime(RECOIL_WASMTIME_TARGET)

set(sources_engine_WasmInterface
	${CMAKE_CURRENT_SOURCE_DIR}/runtime/WasmEnvironment.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/host/WasmCoreAbi.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/bindings/WasmCoreBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/bindings/WasmCoreUnitsInfoVariableBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/bindings/WasmCoreUnitsQueryBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/bindings/WasmCoreUnitsQueryBorrowedBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/bindings/WasmCoreUnitDefsBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/bindings/WasmCoreUnitsCommandsBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/bindings/WasmCoreCobScriptBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/bindings/WasmCoreUnitsPiecesBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/bindings/WasmCoreUnitControlBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/bindings/WasmCoreTerrainControlBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/bindings/WasmCoreSystemControlBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/bindings/WasmCoreMathExtraBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/bindings/WasmCoreTerrainReadBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/bindings/WasmCoreGfxBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/bindings/WasmCoreGfxResourceBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/bindings/WasmCoreVfsBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/bindings/WasmCoreRmlUiBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/bindings/WasmCoreProfilingBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/bindings/WasmCoreMessagesBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/bindings/WasmCoreRulesParamsBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/bindings/WasmCoreConfigBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/bindings/WasmCoreBenchmarkBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/bindings/WasmCoreDesyncBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/bindings/WasmCoreVariableCallins.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/host/WasmCoreHost.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/core/host/WasmCoreValidation.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/runtime/WasmModuleManifest.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/runtime/WasmResources.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/runtime/WasmRuntime.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/system/WasmInterfaceSystem.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/system/WasmInterfaceSystemCore.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/standalone/WasmStandaloneEnvironment.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/standalone/WasmDefsParser.cpp
)

# Generated variable transports are layered by specialization. Nested dynamic
# input is the broad fallback; ordinary adaptation, borrowed zero-copy and
# combined variable-I/O registrations may shadow it at runtime where eligible.
foreach(RECOIL_WASM_CORE_GENERATED_SOURCE
	${CMAKE_CURRENT_SOURCE_DIR}/../wasm/generated/WasmCoreGeneratedBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/../wasm/generated/WasmCoreGeneratedOptionBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/../wasm/generated/WasmCoreGeneratedVariableOutputBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/../wasm/generated/WasmCoreGeneratedDynamicOutputBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/../wasm/generated/WasmCoreGeneratedDynamicInputBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/../wasm/generated/WasmCoreGeneratedVariableBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/../wasm/generated/WasmCoreGeneratedBorrowedBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/../wasm/generated/WasmCoreGeneratedVariableIoBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/../wasm/generated/WasmCoreGeneratedCallinBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/../wasm/generated/WasmCoreGeneratedScratchCallinBindings.cpp
)
	if(EXISTS ${RECOIL_WASM_CORE_GENERATED_SOURCE})
		list(APPEND sources_engine_WasmInterface ${RECOIL_WASM_CORE_GENERATED_SOURCE})
	endif()
endforeach()

file(GLOB_RECURSE RECOIL_WASM_CORE_FIXED_GENERATED_SOURCES CONFIGURE_DEPENDS
	${CMAKE_CURRENT_SOURCE_DIR}/../wasm/generated/host/core/fixed/*.cpp
)
list(APPEND sources_engine_WasmInterface ${RECOIL_WASM_CORE_FIXED_GENERATED_SOURCES})

set(sources_engine_WasmInterface ${sources_engine_WasmInterface} PARENT_SCOPE)

list(APPEND engineCommonLibraries ${RECOIL_WASMTIME_TARGET})
set(engineCommonLibraries "${engineCommonLibraries}" PARENT_SCOPE)
list(APPEND engineIncludes "${CMAKE_CURRENT_SOURCE_DIR}/../wasm")
set(engineIncludes "${engineIncludes}" PARENT_SCOPE)
list(APPEND engineIncludes
	"${CMAKE_CURRENT_SOURCE_DIR}/runtime"
	"${CMAKE_CURRENT_SOURCE_DIR}/core/host"
	"${CMAKE_CURRENT_SOURCE_DIR}/core/bindings"
	"${CMAKE_CURRENT_SOURCE_DIR}/system"
)
set(engineIncludes "${engineIncludes}" PARENT_SCOPE)
set(RECOIL_WASMTIME_TARGET "${RECOIL_WASMTIME_TARGET}" PARENT_SCOPE)
