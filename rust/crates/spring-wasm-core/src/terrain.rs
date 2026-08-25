// Fixed-width Terrain reads for the Spring Core-Wasm guest SDK.

use super::{ApiError, ErrorCode, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapPosition {
    pub in_map: bool,
    pub in_play_area: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GroundNormal {
    pub normal: [f32; 3],
    pub slope: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GroundExtremes {
    pub initial_min: f32,
    pub initial_max: f32,
    pub current_min: f32,
    pub current_max: f32,
}

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:terrain")]
    extern "C" {
        #[link_name = "is-pos-in-map"]
        pub fn is_pos_in_map(x: f32, z: f32) -> i64;
        #[link_name = "get-ground-height"]
        pub fn get_ground_height(x: f32, z: f32) -> i64;
        #[link_name = "get-ground-orig-height"]
        pub fn get_ground_orig_height(x: f32, z: f32) -> i64;
        #[link_name = "get-smooth-mesh-height"]
        pub fn get_smooth_mesh_height(x: f32, z: f32) -> i64;
        #[link_name = "get-water-plane-level"]
        pub fn get_water_plane_level() -> i64;
        #[link_name = "get-water-level"]
        pub fn get_water_level(x: f32, z: f32) -> i64;
        #[link_name = "get-ground-normal"]
        pub fn get_ground_normal(x: f32, z: f32, smoothed: i32, output: i32) -> i32;
        #[link_name = "get-ground-extremes"]
        pub fn get_ground_extremes(output: i32) -> i32;
        #[link_name = "get-height-map-size"]
        pub fn get_height_map_size(output: i32) -> i32;
        #[link_name = "get-ground-blocked"]
        pub fn get_ground_blocked(x1: f32, z1: f32, x2: f32, z2: f32) -> i64;
        #[link_name = "get-grass"]
        pub fn get_grass(x: f32, z: f32) -> i64;
    }
}

#[inline]
pub fn is_pos_in_map(x: f32, z: f32) -> Result<MapPosition> {
    #[cfg(target_arch = "wasm32")]
    {
        let packed = unsafe { raw::is_pos_in_map(x, z) } as u64;
        let status = (packed >> 32) as u32 as i32;
        if status != 0 {
            return Err(ApiError::new(status));
        }
        let flags = packed as u32;
        if flags & !0x3 != 0 {
            return Err(ApiError::new(ErrorCode::Internal as i32));
        }
        Ok(MapPosition {
            in_map: flags & 0x1 != 0,
            in_play_area: flags & 0x2 != 0,
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (x, z);
        Err(unreachable!())
    }
}

macro_rules! scalar_f32_query {
    ($name:ident, $raw:ident) => {
        #[inline]
        pub fn $name(x: f32, z: f32) -> Result<f32> {
            #[cfg(target_arch = "wasm32")]
            {
                return super::unpack_f32(unsafe { raw::$raw(x, z) });
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = (x, z);
                Err(unreachable!())
            }
        }
    };
}

scalar_f32_query!(get_ground_height, get_ground_height);
scalar_f32_query!(get_ground_orig_height, get_ground_orig_height);
scalar_f32_query!(get_smooth_mesh_height, get_smooth_mesh_height);
scalar_f32_query!(get_water_level, get_water_level);
scalar_f32_query!(get_grass, get_grass);

#[inline]
pub fn get_water_plane_level() -> Result<f32> {
    #[cfg(target_arch = "wasm32")]
    {
        super::unpack_f32(unsafe { raw::get_water_plane_level() })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Err(unreachable!())
    }
}

#[inline]
pub fn get_ground_blocked(x1: f32, z1: f32, x2: f32, z2: f32) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        super::unpack_bool(unsafe { raw::get_ground_blocked(x1, z1, x2, z2) })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (x1, z1, x2, z2);
        Err(unreachable!())
    }
}

#[inline]
pub fn get_height_map_size() -> Result<[i32; 2]> {
    #[cfg(target_arch = "wasm32")]
    {
        let mut output = [0i32; 2];
        let pointer = output.as_mut_ptr() as usize;
        if pointer > u32::MAX as usize {
            return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
        }
        let status = unsafe { raw::get_height_map_size(pointer as u32 as i32) };
        if status == 0 {
            Ok(output)
        } else {
            Err(ApiError::new(status))
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Err(unreachable!())
    }
}

#[inline]
pub fn get_ground_extremes() -> Result<GroundExtremes> {
    #[cfg(target_arch = "wasm32")]
    {
        let mut output = [0.0f32; 4];
        let pointer = output.as_mut_ptr() as usize;
        if pointer > u32::MAX as usize {
            return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
        }
        let status = unsafe { raw::get_ground_extremes(pointer as u32 as i32) };
        if status != 0 {
            return Err(ApiError::new(status));
        }
        Ok(GroundExtremes {
            initial_min: output[0],
            initial_max: output[1],
            current_min: output[2],
            current_max: output[3],
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Err(unreachable!())
    }
}

#[inline]
pub fn get_ground_normal(x: f32, z: f32, smoothed: bool) -> Result<GroundNormal> {
    #[cfg(target_arch = "wasm32")]
    {
        let mut output = [0.0f32; 4];
        let pointer = output.as_mut_ptr() as usize;
        if pointer > u32::MAX as usize {
            return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
        }
        let status =
            unsafe { raw::get_ground_normal(x, z, smoothed as i32, pointer as u32 as i32) };
        if status != 0 {
            return Err(ApiError::new(status));
        }
        Ok(GroundNormal {
            normal: [output[0], output[1], output[2]],
            slope: output[3],
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (x, z, smoothed);
        Err(unreachable!())
    }
}
