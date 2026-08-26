

#[cfg(feature = "alloc")]
pub use crate::owned::math_extra::{bit_and, bit_bits, bit_inv, bit_or, bit_xor, clamp, diag, erf, hypot, mix, round, sgn, smooth_step};

use super::{ApiError, ErrorCode, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NormalizedVec3 {
    pub vector: [f32; 3],
    pub length: f32,
}

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:math-extra")]
    unsafe extern "C" {
        pub fn normalize(x: f32, y: f32, z: f32, output: i32) -> i32;
    }
}

#[inline]
pub fn normalize(vector: [f32; 3]) -> Result<NormalizedVec3> {
    #[cfg(target_arch = "wasm32")]
    {
        let mut output = [0.0f32; 4];
        let pointer = output.as_mut_ptr() as usize;
        if pointer > u32::MAX as usize {
            return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
        }
        let status =
            unsafe { raw::normalize(vector[0], vector[1], vector[2], pointer as u32 as i32) };
        if status == 0 {
            Ok(NormalizedVec3 {
                vector: [output[0], output[1], output[2]],
                length: output[3],
            })
        } else {
            Err(ApiError::new(status))
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = vector;
        Err(unreachable!())
    }
}
