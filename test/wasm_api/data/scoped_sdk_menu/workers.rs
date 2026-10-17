/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */
pub fn double(bytes: &[u8]) -> Vec<u8> {
    bytes.iter().map(|b| b * 2).collect()
}
pub fn oversized(_: &[u8]) -> Vec<u8> {
    vec![0; spring_sdk::jobs::MAX_BYTES + 1]
}
