include(${CMAKE_CURRENT_SOURCE_DIR}/WasmtimeDependency.cmake)
recoil_wasm_configure_wasmtime(RECOIL_WASMTIME_TARGET)

set(sources_engine_WasmInterface
	${CMAKE_CURRENT_SOURCE_DIR}/WasmEnvironment.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmHost.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreAbi.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreUnitsInfoVariableBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreUnitsQueryBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreUnitsQueryBorrowedBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreUnitDefsBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreUnitsCommandsBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreCobScriptBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreUnitsPiecesBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreUnitControlBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreTerrainControlBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreSystemControlBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreMathExtraBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreTerrainReadBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreGfxBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreGfxResourceBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreVfsBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreRmlUiBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreProfilingBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreMessagesBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreRulesParamsBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreConfigBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreBenchmarkBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreDesyncBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreVariableCallins.cpp
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
	${CMAKE_CURRENT_SOURCE_DIR}/WasmInterfaceSystemCore.cpp
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

set(sources_engine_WasmInterface ${sources_engine_WasmInterface} PARENT_SCOPE)

list(APPEND engineCommonLibraries ${RECOIL_WASMTIME_TARGET})
set(engineCommonLibraries "${engineCommonLibraries}" PARENT_SCOPE)
list(APPEND engineIncludes "${CMAKE_CURRENT_SOURCE_DIR}/../wasm")
set(engineIncludes "${engineIncludes}" PARENT_SCOPE)
set(RECOIL_WASMTIME_TARGET "${RECOIL_WASMTIME_TARGET}" PARENT_SCOPE)
