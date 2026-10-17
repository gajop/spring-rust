/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

// Engine-owned graphics handles can outlive their modules. Release them while
// the GL context is current, rather than during static C++ destruction.
void CleanupNativeGfxResources();
