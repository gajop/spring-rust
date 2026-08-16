# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

# Resolve the exact Wasmtime C API release used by the engine.  Packaged builds
# should set WASMTIME_ROOT to a provisioned SDK; source builds download the
# checksum-pinned release into the writable CMake build tree.
function(recoil_wasm_configure_wasmtime out_target)
	set(version_file "${CMAKE_SOURCE_DIR}/rts/wasm/wasmtime.version")
	file(READ "${version_file}" version_text)
	string(STRIP "${version_text}" version)

	if(WIN32)
		if(CMAKE_SIZEOF_VOID_P EQUAL 8)
			set(asset "wasmtime-v${version}-x86_64-windows-c-api.zip")
		elseif(CMAKE_SIZEOF_VOID_P EQUAL 4)
			set(asset "wasmtime-v${version}-i686-windows-c-api.zip")
		else()
			message(FATAL_ERROR "Unsupported Windows pointer size for Wasmtime: ${CMAKE_SIZEOF_VOID_P}")
		endif()
	elseif(CMAKE_SYSTEM_PROCESSOR MATCHES "^(aarch64|arm64)$")
		set(asset "wasmtime-v${version}-aarch64-linux-c-api.tar.xz")
	else()
		set(asset "wasmtime-v${version}-x86_64-linux-c-api.tar.xz")
	endif()

	set(root "${WASMTIME_ROOT}")
	if(root)
		if(NOT EXISTS "${root}/include/wasmtime.h")
			message(FATAL_ERROR "WASMTIME_ROOT does not contain include/wasmtime.h: ${root}")
		endif()
	else()
		set(checksum_file "${CMAKE_SOURCE_DIR}/rts/wasm/wasmtime.sha256")
		file(STRINGS "${checksum_file}" checksum_lines)
		set(checksum_line "")
		foreach(candidate IN LISTS checksum_lines)
			if(candidate MATCHES "^${asset} ")
				if(checksum_line)
					message(FATAL_ERROR "Multiple checksums are recorded for Wasmtime artifact ${asset}")
				endif()
				set(checksum_line "${candidate}")
			endif()
		endforeach()
		if(NOT checksum_line)
			message(FATAL_ERROR "No unique checksum is recorded for Wasmtime artifact ${asset}")
		endif()
		string(REGEX MATCH "^[^ ]+ ([0-9a-fA-F]+)$" unused "${checksum_line}")
		set(checksum "${CMAKE_MATCH_1}")

		set(download_dir "${CMAKE_BINARY_DIR}/_deps/wasmtime-${version}")
		string(REGEX REPLACE "\\.(tar\\.xz|zip)$" "" archive_stem "${asset}")
		set(root "${download_dir}/${archive_stem}")

		if(NOT EXISTS "${root}/include/wasmtime.h")
			file(MAKE_DIRECTORY "${download_dir}")
			set(archive "${download_dir}/${asset}")
			file(DOWNLOAD
				https://github.com/bytecodealliance/wasmtime/releases/download/v${version}/${asset}
				"${archive}"
				EXPECTED_HASH "SHA256=${checksum}"
				STATUS download_status
				SHOW_PROGRESS
			)
			list(GET download_status 0 download_code)
			if(NOT download_code EQUAL 0)
				list(GET download_status 1 download_message)
				message(FATAL_ERROR "Could not download Wasmtime ${version}: ${download_message}")
			endif()
			file(ARCHIVE_EXTRACT INPUT "${archive}" DESTINATION "${download_dir}")
		endif()
	endif()

	set(wasmtime_runtime_dll "")
	if(MINGW AND EXISTS "${root}/lib/wasmtime.dll.lib" AND EXISTS "${root}/lib/wasmtime.dll")
		# The Windows C API archive also contains a large static Rust library
		# built for the MSVC ABI.  MinGW cannot link that archive, but the
		# package's DLL/import-library pair is COFF-compatible and keeps the
		# Rust runtime behind the DLL boundary.
		set(library "${root}/lib/wasmtime.dll.lib")
		set(wasmtime_runtime_dll "${root}/lib/wasmtime.dll")
	elseif(WIN32)
		set(library "${root}/lib/wasmtime.lib")
		if(NOT EXISTS "${library}")
			set(library "${root}/lib/libwasmtime.a")
		endif()
	else()
		set(library "${root}/lib/libwasmtime.a")
	endif()
	if(NOT EXISTS "${library}")
		message(FATAL_ERROR "Wasmtime C API library was not found under ${root}")
	endif()

	set(wasmtime_link_libraries "Threads::Threads;${CMAKE_DL_LIBS}")
	if(UNIX)
		list(APPEND wasmtime_link_libraries m)
	endif()
	if(NOT TARGET recoil-wasmtime)
		if(wasmtime_runtime_dll)
			add_library(recoil-wasmtime SHARED IMPORTED GLOBAL)
			set_target_properties(recoil-wasmtime PROPERTIES
				IMPORTED_IMPLIB "${library}"
				IMPORTED_LOCATION "${wasmtime_runtime_dll}"
			)
		else()
			add_library(recoil-wasmtime STATIC IMPORTED GLOBAL)
			set_target_properties(recoil-wasmtime PROPERTIES
				IMPORTED_LOCATION "${library}"
			)
		endif()
		set_target_properties(recoil-wasmtime PROPERTIES
			INTERFACE_INCLUDE_DIRECTORIES "${root}/include"
			INTERFACE_COMPILE_DEFINITIONS "RECOIL_WASMTIME_AVAILABLE=1;LIBWASM_STATIC=1"
			INTERFACE_LINK_LIBRARIES "${wasmtime_link_libraries}"
		)
		if(wasmtime_runtime_dll)
			install(FILES "${wasmtime_runtime_dll}" DESTINATION "${BINDIR}")
		endif()
	endif()

	set(${out_target} recoil-wasmtime PARENT_SCOPE)
endfunction()
