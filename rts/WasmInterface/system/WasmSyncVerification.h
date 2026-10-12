/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

class WasmSyncVerification {
public:
	// After all synced callins, including GameFramePost; before leaving synced code.
	static void EndFrame(int frame);
};
