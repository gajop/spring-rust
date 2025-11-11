#include "SyncedCtrl.h"

namespace {

static const Error NOT_IMPLEMENTED_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "SyncedCtrl API not yet fully implemented - stubs only"
};

// Synced control functions - all stubbed
// Would provide game state modification (unit commands, etc.)
// This is very complex and requires careful syncing

} // namespace

const SyncedCtrlApi SYNCED_CTRL_API = {
	// Function pointers stubbed
};
