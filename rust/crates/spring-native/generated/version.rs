// API version constants extracted from Common.h at build time.

/// Major version of the Native API this module was built against.
///
/// Major version MUST match between host and module for compatibility.
pub const NATIVE_API_VERSION_MAJOR: u32 = 1;

/// Minor version of the Native API this module was built against.
///
/// Module can require a minimum minor version from the host.
pub const NATIVE_API_VERSION_MINOR: u32 = 5;

/// Patch version of the Native API this module was built against.
pub const NATIVE_API_VERSION_PATCH: u32 = 0;
