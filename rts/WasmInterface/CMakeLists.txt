include(${CMAKE_CURRENT_SOURCE_DIR}/WasmtimeDependency.cmake)
recoil_wasm_configure_wasmtime(RECOIL_WASMTIME_TARGET)

set(sources_engine_WasmInterface
	${CMAKE_CURRENT_SOURCE_DIR}/WasmEnvironment.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmHost.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreAbi.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreBindings.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmCoreHost.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmModule.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmModuleManifest.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmDispatch.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmResources.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmRuntime.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmTypedHost.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmTypedHostShims.cpp
	${CMAKE_CURRENT_SOURCE_DIR}/WasmInterfaceSystem.cpp
	PARENT_SCOPE
)

list(APPEND engineCommonLibraries ${RECOIL_WASMTIME_TARGET})
set(engineCommonLibraries "${engineCommonLibraries}" PARENT_SCOPE)
list(APPEND engineIncludes "${CMAKE_CURRENT_SOURCE_DIR}/../wasm")
set(engineIncludes "${engineIncludes}" PARENT_SCOPE)
set(RECOIL_WASMTIME_TARGET "${RECOIL_WASMTIME_TARGET}" PARENT_SCOPE)
