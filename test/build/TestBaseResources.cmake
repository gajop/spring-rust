# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

if (NOT DEFINED SOURCE_DIR)
	get_filename_component(SOURCE_DIR "${CMAKE_CURRENT_LIST_DIR}/../.." ABSOLUTE)
endif()

set(RESOURCE_MANIFEST "${SOURCE_DIR}/cont/base/springcontent/gamedata/resources.lua")
set(BITMAP_MANIFEST "${SOURCE_DIR}/cont/base/bitmaps/CMakeLists.txt")

file(READ "${RESOURCE_MANIFEST}" resources)
string(REGEX MATCHALL "'scars/[^']+'" scar_entries "${resources}")

if (NOT scar_entries)
	message(FATAL_ERROR "No scar resources found in ${RESOURCE_MANIFEST}")
endif()

foreach (entry IN LISTS scar_entries)
	string(REGEX REPLACE "^'|'$" "" relative_path "${entry}")
	if (NOT EXISTS "${SOURCE_DIR}/cont/base/bitmaps/bitmaps/${relative_path}")
		message(FATAL_ERROR "Resource manifest entry does not exist: ${relative_path}")
	endif()
endforeach()

file(READ "${BITMAP_MANIFEST}" bitmap_files)
foreach (track IN ITEMS
		bitmaps/tracks/ComTrack_normal.bmp
		bitmaps/tracks/StdTank_normal.bmp
)
	string(FIND "${bitmap_files}" "${track}" track_offset)
	if (track_offset EQUAL -1)
		message(FATAL_ERROR "Track normal map is not packaged: ${track}")
	endif()
endforeach()

message(STATUS "base resource references and track normal maps are present")
