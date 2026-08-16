impl<'a> MathExtra<'a> {
    pub fn hypot(&self, x: f32, y: f32) -> Result<f32, Error> {
        unsafe {
            let query = sys::HypotQuery {
                x: x,
                y: y,
            };
            let mut result = MaybeUninit::<sys::HypotResult>::zeroed();
            let func = self.api.Hypot.expect("Hypot function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn diag(&self, values: &[f32]) -> Result<f32, Error> {
        unsafe {
            let query = sys::DiagQuery {
                values: values.as_ptr(),
                count: values.len() as u32,
            };
            let mut result = MaybeUninit::<sys::DiagResult>::zeroed();
            let func = self.api.Diag.expect("Diag function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.length
            })
        }
    }

    pub fn clamp(&self, value: f32, min: f32, max: f32) -> Result<f32, Error> {
        unsafe {
            let query = sys::ClampQuery {
                value: value,
                min: min,
                max: max,
            };
            let mut result = MaybeUninit::<sys::ClampResult>::zeroed();
            let func = self.api.Clamp.expect("Clamp function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.clamped
            })
        }
    }

    pub fn sgn(&self, value: f32) -> Result<f32, Error> {
        unsafe {
            let query = sys::SgnQuery {
                value: value,
            };
            let mut result = MaybeUninit::<sys::SgnResult>::zeroed();
            let func = self.api.Sgn.expect("Sgn function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.sign
            })
        }
    }

    pub fn mix(&self, a: f32, b: f32, t: f32) -> Result<f32, Error> {
        unsafe {
            let query = sys::MixQuery {
                a: a,
                b: b,
                t: t,
            };
            let mut result = MaybeUninit::<sys::MixResult>::zeroed();
            let func = self.api.Mix.expect("Mix function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.mixed
            })
        }
    }

    pub fn round(&self, value: f32) -> Result<f32, Error> {
        unsafe {
            let query = sys::RoundQuery {
                value: value,
            };
            let mut result = MaybeUninit::<sys::RoundResult>::zeroed();
            let func = self.api.Round.expect("Round function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.rounded
            })
        }
    }

    pub fn erf(&self, value: f32) -> Result<f32, Error> {
        unsafe {
            let query = sys::ErfQuery {
                value: value,
            };
            let mut result = MaybeUninit::<sys::ErfResult>::zeroed();
            let func = self.api.Erf.expect("Erf function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.result
            })
        }
    }

    pub fn smooth_step(&self, edge0: f32, edge1: f32, x: f32) -> Result<f32, Error> {
        unsafe {
            let query = sys::SmoothStepQuery {
                edge0: edge0,
                edge1: edge1,
                x: x,
            };
            let mut result = MaybeUninit::<sys::SmoothStepResult>::zeroed();
            let func = self.api.SmoothStep.expect("SmoothStep function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn normalize(&self, vec: &mut sys::Float3) -> Result<f32, Error> {
        unsafe {
            let query = sys::NormalizeQuery {
                vec: vec as *mut _,
            };
            let mut result = MaybeUninit::<sys::NormalizeResult>::zeroed();
            let func = self.api.Normalize.expect("Normalize function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.length
            })
        }
    }

    pub fn bit_or(&self, a: u32, b: u32) -> Result<u32, Error> {
        unsafe {
            let query = sys::BitOrQuery {
                a: a,
                b: b,
            };
            let mut result = MaybeUninit::<sys::BitOrResult>::zeroed();
            let func = self.api.BitOr.expect("BitOr function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn bit_and(&self, a: u32, b: u32) -> Result<u32, Error> {
        unsafe {
            let query = sys::BitAndQuery {
                a: a,
                b: b,
            };
            let mut result = MaybeUninit::<sys::BitAndResult>::zeroed();
            let func = self.api.BitAnd.expect("BitAnd function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn bit_xor(&self, a: u32, b: u32) -> Result<u32, Error> {
        unsafe {
            let query = sys::BitXorQuery {
                a: a,
                b: b,
            };
            let mut result = MaybeUninit::<sys::BitXorResult>::zeroed();
            let func = self.api.BitXor.expect("BitXor function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn bit_inv(&self, a: u32) -> Result<u32, Error> {
        unsafe {
            let query = sys::BitInvQuery {
                a: a,
            };
            let mut result = MaybeUninit::<sys::BitInvResult>::zeroed();
            let func = self.api.BitInv.expect("BitInv function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.value
            })
        }
    }

    pub fn bit_bits(&self, bits: &[u32]) -> Result<u32, Error> {
        unsafe {
            let query = sys::BitBitsQuery {
                bits: bits.as_ptr(),
                count: bits.len() as u32,
            };
            let mut result = MaybeUninit::<sys::BitBitsResult>::zeroed();
            let func = self.api.BitBits.expect("BitBits function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.bits
            })
        }
    }

}
