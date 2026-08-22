    pub mod metal_map {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, PartialEq)]
        pub struct Error {
            pub code: i32,
            pub message: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMetalAmountQuery {
            pub x: i32,
            pub z: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMetalAmountResult {
            pub amount: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMetalExtractionQuery {
            pub x: i32,
            pub z: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMetalExtractionResult {
            pub extraction: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMetalMapSizeQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMetalMapSizeResult {
            pub width: i32,
            pub height: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetMetalAmountQuery {
            pub x: i32,
            pub z: i32,
            pub amount: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetMetalAmountResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMetalMapSizeValue {
            pub width: i32,
            pub height: i32,
        }

        #[inline]
        pub fn get_metal_amount(x: i32, z: i32) -> Result<f32> {
            let value = crate::generated::metal_map::get_metal_amount(x, z)?;
            Ok(value)
        }

        #[inline]
        pub fn get_metal_extraction(x: i32, z: i32) -> Result<f32> {
            let value = crate::generated::metal_map::get_metal_extraction(x, z)?;
            Ok(value)
        }

        #[inline]
        pub fn get_metal_map_size(unused: u8) -> Result<GetMetalMapSizeValue> {
            let value = crate::generated::metal_map::get_metal_map_size(unused)?;
            Ok(GetMetalMapSizeValue {
                width: value.0,
                height: value.1
            })
        }

        #[inline]
        pub fn set_metal_amount(x: i32, z: i32, amount: f32) -> Result<()> {
            crate::generated::metal_map::set_metal_amount(x, z, amount)?;
            Ok(())
        }

    }

