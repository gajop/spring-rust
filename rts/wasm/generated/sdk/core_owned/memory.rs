    pub mod memory {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, PartialEq)]
        pub struct FreeFloat2ArrayQuery {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FreeFloat2ArrayResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FreeFloat3ArrayQuery {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FreeFloat3ArrayResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FreeFloat4ArrayQuery {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FreeFloat4ArrayResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FreeFloatArrayQuery {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FreeFloatArrayResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FreeInt32ArrayQuery {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FreeInt32ArrayResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FreeInt3ArrayQuery {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FreeInt3ArrayResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FreeQuery {
            pub ptr: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FreeResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FreeStringArrayQuery {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FreeStringArrayResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FreeUInt32ArrayQuery {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FreeUInt32ArrayResult {
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

    }

