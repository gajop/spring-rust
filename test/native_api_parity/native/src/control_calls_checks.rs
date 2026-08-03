use super::*;
use crate::support::*;

fn light_array3(value: &Value, field: &str) -> Result<[f32; 3], String> {
    let values = value
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("missing light parameter array `{field}`"))?;
    if values.len() < 3 {
        return Err(format!(
            "light parameter `{field}` has fewer than three values"
        ));
    }
    Ok([
        values[0]
            .as_f64()
            .ok_or_else(|| format!("light parameter `{field}[0]` is not numeric"))? as f32,
        values[1]
            .as_f64()
            .ok_or_else(|| format!("light parameter `{field}[1]` is not numeric"))? as f32,
        values[2]
            .as_f64()
            .ok_or_else(|| format!("light parameter `{field}[2]` is not numeric"))? as f32,
    ])
}

fn light_f32(value: &Value, field: &str) -> Result<f32, String> {
    value
        .get(field)
        .and_then(Value::as_f64)
        .map(|value| value as f32)
        .ok_or_else(|| format!("missing light parameter `{field}`"))
}

fn light_u32(value: &Value, field: &str) -> Result<u32, String> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .and_then(|value| u32::try_from(value).ok())
        .ok_or_else(|| format!("missing unsigned light parameter `{field}`"))
}

fn light_bool(value: &Value, field: &str) -> Result<bool, String> {
    value
        .get(field)
        .and_then(Value::as_bool)
        .ok_or_else(|| format!("missing boolean light parameter `{field}`"))
}

fn lua_light_handle(message: &Value, field: &str) -> Result<u32, String> {
    let value = message
        .get(field)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("missing unsigned integer field `{field}`"))?;
    Ok(if value >= u32::MAX as u64 {
        u32::MAX
    } else {
        value as u32
    })
}

fn light_params(message: &Value) -> Result<sys::LightParams, String> {
    let value = message
        .get("lightParams")
        .ok_or_else(|| "missing lightParams object".to_string())?;
    Ok(sys::LightParams {
        position: light_array3(value, "position")?,
        direction: light_array3(value, "direction")?,
        ambientColor: light_array3(value, "ambientColor")?,
        diffuseColor: light_array3(value, "diffuseColor")?,
        specularColor: light_array3(value, "specularColor")?,
        intensityWeight: [0.0; 3],
        attenuation: [0.0; 3],
        ambientDecayRate: [0.0; 3],
        diffuseDecayRate: [0.0; 3],
        specularDecayRate: [0.0; 3],
        decayFunctionType: [0.0; 3],
        radius: light_f32(value, "radius")?,
        fov: light_f32(value, "fov")?,
        ttl: light_u32(value, "ttl")?,
        priority: light_u32(value, "priority")?,
        ignoreLOS: light_bool(value, "ignoreLOS")?,
        localSpace: light_bool(value, "localSpace")?,
    })
}

fn light_result(label: &str, result: Result<bool, spring_native::Error>) -> Result<bool, String> {
    result.map_err(|err| format!("{label} failed: {err:?}"))
}

fn require_native_success(
    label: &str,
    result: Result<bool, spring_native::Error>,
) -> Result<(), String> {
    if result.map_err(|err| format!("{label} failed: {err:?}"))? {
        Ok(())
    } else {
        Err(format!("{label} returned false"))
    }
}

fn parameter_object<'a>(message: &'a Value, field: &str) -> Result<&'a Value, String> {
    let value = message
        .get(field)
        .ok_or_else(|| format!("missing object parameter `{field}`"))?;
    if !value.is_object() {
        return Err(format!("parameter `{field}` is not an object"));
    }
    Ok(value)
}

fn parameter_array<const N: usize>(object: &Value, field: &str) -> Result<[f32; N], String> {
    let values = object
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| format!("missing parameter array `{field}`"))?;
    if values.len() < N {
        return Err(format!(
            "parameter array `{field}` has fewer than {N} values"
        ));
    }
    let mut result = [0.0; N];
    for (index, value) in values.iter().take(N).enumerate() {
        result[index] = value
            .as_f64()
            .ok_or_else(|| format!("parameter array `{field}[{index}]` is not numeric"))?
            as f32;
    }
    Ok(result)
}

fn parameter_f32(object: &Value, field: &str) -> Result<f32, String> {
    object
        .get(field)
        .and_then(Value::as_f64)
        .map(|value| value as f32)
        .ok_or_else(|| format!("missing numeric parameter `{field}`"))
}

fn parameter_bool(object: &Value, field: &str) -> Result<bool, String> {
    object
        .get(field)
        .and_then(Value::as_bool)
        .ok_or_else(|| format!("missing boolean parameter `{field}`"))
}

fn atmosphere_params(message: &Value) -> Result<sys::AtmosphereParams, String> {
    let value = parameter_object(message, "atmosphere")?;
    let mut params = sys::AtmosphereParams::default();
    params.fogColor = parameter_array(value, "fogColor")?;
    params.hasFogColor = true;
    params.skyColor = parameter_array(value, "skyColor")?;
    params.hasSkyColor = true;
    params.sunColor = parameter_array(value, "sunColor")?;
    params.hasSunColor = true;
    params.cloudColor = parameter_array(value, "cloudColor")?;
    params.hasCloudColor = true;
    params.skyAxisAngle = parameter_array(value, "skyAxisAngle")?;
    params.hasSkyAxisAngle = true;
    params.fogStart = parameter_f32(value, "fogStart")?;
    params.hasFogStart = true;
    params.fogEnd = parameter_f32(value, "fogEnd")?;
    params.hasFogEnd = true;
    Ok(params)
}

fn sun_lighting_params(message: &Value) -> Result<sys::SunLightingParams, String> {
    let value = parameter_object(message, "lighting")?;
    let mut params = sys::SunLightingParams::default();
    params.groundAmbientColor = parameter_array(value, "groundAmbientColor")?;
    params.hasGroundAmbientColor = true;
    params.groundDiffuseColor = parameter_array(value, "groundDiffuseColor")?;
    params.hasGroundDiffuseColor = true;
    params.groundSpecularColor = parameter_array(value, "groundSpecularColor")?;
    params.hasGroundSpecularColor = true;
    params.modelAmbientColor = parameter_array(value, "modelAmbientColor")?;
    params.hasModelAmbientColor = true;
    params.modelDiffuseColor = parameter_array(value, "modelDiffuseColor")?;
    params.hasModelDiffuseColor = true;
    params.modelSpecularColor = parameter_array(value, "modelSpecularColor")?;
    params.hasModelSpecularColor = true;
    params.specularExponent = parameter_f32(value, "specularExponent")?;
    params.hasSpecularExponent = true;
    params.groundShadowDensity = parameter_f32(value, "groundShadowDensity")?;
    params.hasGroundShadowDensity = true;
    params.modelShadowDensity = parameter_f32(value, "modelShadowDensity")?;
    params.hasModelShadowDensity = true;
    Ok(params)
}

fn water_params(message: &Value) -> Result<sys::WaterParams, String> {
    let value = parameter_object(message, "water")?;
    let mut params = sys::WaterParams::default();

    macro_rules! array3 {
        ($name:literal, $field:ident, $has_field:ident) => {
            params.$field = parameter_array(value, $name)?;
            params.$has_field = true;
        };
    }
    macro_rules! scalar {
        ($name:literal, $field:ident, $has_field:ident) => {
            params.$field = parameter_f32(value, $name)?;
            params.$has_field = true;
        };
    }
    macro_rules! boolean {
        ($name:literal, $field:ident, $has_field:ident) => {
            params.$field = parameter_bool(value, $name)?;
            params.$has_field = true;
        };
    }

    array3!("absorb", absorb, hasAbsorb);
    array3!("baseColor", baseColor, hasBaseColor);
    array3!("minColor", minColor, hasMinColor);
    array3!("surfaceColor", surfaceColor, hasSurfaceColor);
    array3!("diffuseColor", diffuseColor, hasDiffuseColor);
    array3!("specularColor", specularColor, hasSpecularColor);
    array3!("planeColor", planeColor, hasPlaneColor);
    scalar!("repeatX", repeatX, hasRepeatX);
    scalar!("repeatY", repeatY, hasRepeatY);
    scalar!("surfaceAlpha", surfaceAlpha, hasSurfaceAlpha);
    scalar!("ambientFactor", ambientFactor, hasAmbientFactor);
    scalar!("diffuseFactor", diffuseFactor, hasDiffuseFactor);
    scalar!("specularFactor", specularFactor, hasSpecularFactor);
    scalar!("specularPower", specularPower, hasSpecularPower);
    scalar!("fresnelMin", fresnelMin, hasFresnelMin);
    scalar!("fresnelMax", fresnelMax, hasFresnelMax);
    scalar!("fresnelPower", fresnelPower, hasFresnelPower);
    scalar!(
        "reflectionDistortion",
        reflectionDistortion,
        hasReflectionDistortion
    );
    scalar!("blurBase", blurBase, hasBlurBase);
    scalar!("blurExponent", blurExponent, hasBlurExponent);
    scalar!("perlinStartFreq", perlinStartFreq, hasPerlinStartFreq);
    scalar!("perlinLacunarity", perlinLacunarity, hasPerlinLacunarity);
    scalar!("perlinAmplitude", perlinAmplitude, hasPerlinAmplitude);
    scalar!("windSpeed", windSpeed, hasWindSpeed);
    scalar!("waveOffsetFactor", waveOffsetFactor, hasWaveOffsetFactor);
    scalar!("waveLength", waveLength, hasWaveLength);
    scalar!(
        "waveFoamDistortion",
        waveFoamDistortion,
        hasWaveFoamDistortion
    );
    scalar!("waveFoamIntensity", waveFoamIntensity, hasWaveFoamIntensity);
    scalar!(
        "causticsResolution",
        causticsResolution,
        hasCausticsResolution
    );
    scalar!("causticsStrength", causticsStrength, hasCausticsStrength);
    scalar!("numTiles", numTiles, hasNumTiles);
    boolean!("shoreWaves", shoreWaves, hasShoreWaves);
    boolean!("forceRendering", forceRendering, hasForceRendering);
    params.hasWaterPlane = parameter_bool(value, "hasWaterPlane")?;
    params.hasHasWaterPlane = true;
    Ok(params)
}

fn map_rendering_params(message: &Value) -> Result<sys::MapRenderingParams, String> {
    let value = parameter_object(message, "mapRendering")?;
    let mut params = sys::MapRenderingParams::default();
    params.splatTexScales = parameter_array(value, "splatTexScales")?;
    params.hasSplatTexScales = true;
    params.splatTexMults = parameter_array(value, "splatTexMults")?;
    params.hasSplatTexMults = true;
    params.voidWater = parameter_bool(value, "voidWater")?;
    params.hasVoidWater = true;
    params.voidGround = parameter_bool(value, "voidGround")?;
    params.hasVoidGround = true;
    params.splatDetailNormalDiffuseAlpha = parameter_bool(value, "splatDetailNormalDiffuseAlpha")?;
    params.hasSplatDetailNormalDiffuseAlpha = true;
    Ok(params)
}

impl NativeApiParity {
    pub(crate) fn check_control_calls(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        match base_test_name(label) {
            "set_atmosphere_params" => require_native_success(
                "set_atmosphere_params",
                self.interface
                    .unsynced_ctrl()
                    .set_atmosphere(atmosphere_params(message)?),
            ),
            "set_sun_lighting_params" => require_native_success(
                "set_sun_lighting_params",
                self.interface
                    .unsynced_ctrl()
                    .set_sun_lighting(sun_lighting_params(message)?),
            ),
            "set_water_params" => {
                require_native_success(
                    "set_water_params",
                    self.interface
                        .unsynced_ctrl()
                        .set_water_params(water_params(message)?),
                )?;
                let water = parameter_object(message, "water")?;
                for (field, key) in [
                    ("texture", "texture"),
                    ("foamTexture", "foamTexture"),
                    ("normalTexture", "normalTexture"),
                ] {
                    require_native_success(
                        field,
                        self.interface.unsynced_ctrl().set_water_texture(
                            field,
                            water
                                .get(key)
                                .and_then(Value::as_str)
                                .ok_or_else(|| format!("missing water string `{key}`"))?,
                        ),
                    )?;
                }
                Ok(())
            }
            "set_map_rendering_params" => require_native_success(
                "set_map_rendering_params",
                self.interface
                    .unsynced_ctrl()
                    .set_map_rendering_params(map_rendering_params(message)?),
            ),
            "map_model_lights_lifecycle" => {
                let params = light_params(message)?;
                let unit_id = i32_field(message, "unitID")?;

                let _map_handle_first = self
                    .interface
                    .lights()
                    .add_map_light(params)
                    .map_err(|err| format!("add_map_light() failed: {err:?}"))?;
                let map_handle_second = self
                    .interface
                    .lights()
                    .add_map_light(params)
                    .map_err(|err| format!("add_map_light(second) failed: {err:?}"))?;
                let map_added = map_handle_second != u32::MAX;
                let map_handle = lua_light_handle(message, "mapHandle")?;
                let map_updated = light_result(
                    "update_map_light",
                    self.interface.lights().update_map_light(map_handle, params),
                )?;
                let map_tracked = light_result(
                    "set_map_light_tracking_state(enable)",
                    self.interface
                        .lights()
                        .set_map_light_tracking_state(map_handle, unit_id, true, true),
                )?;
                let map_untracked = light_result(
                    "set_map_light_tracking_state(disable)",
                    self.interface
                        .lights()
                        .set_map_light_tracking_state(map_handle, unit_id, false, true),
                )?;

                let _model_handle_first = self
                    .interface
                    .lights()
                    .add_model_light(params)
                    .map_err(|err| format!("add_model_light() failed: {err:?}"))?;
                let model_handle_second = self
                    .interface
                    .lights()
                    .add_model_light(params)
                    .map_err(|err| format!("add_model_light(second) failed: {err:?}"))?;
                let model_added = model_handle_second != u32::MAX;
                let model_handle = lua_light_handle(message, "modelHandle")?;
                let model_updated = light_result(
                    "update_model_light",
                    self.interface
                        .lights()
                        .update_model_light(model_handle, params),
                )?;
                let model_tracked = light_result(
                    "set_model_light_tracking_state(enable)",
                    self.interface.lights().set_model_light_tracking_state(
                        model_handle,
                        unit_id,
                        true,
                        true,
                    ),
                )?;
                let model_untracked = light_result(
                    "set_model_light_tracking_state(disable)",
                    self.interface.lights().set_model_light_tracking_state(
                        model_handle,
                        unit_id,
                        false,
                        true,
                    ),
                )?;

                self.same_bool_if_present(label, message, "mapAdded", map_added)?;
                self.same_bool_if_present(label, message, "mapUpdated", map_updated)?;
                self.same_bool_if_present(label, message, "mapTracked", map_tracked)?;
                self.same_bool_if_present(label, message, "mapUntracked", map_untracked)?;
                self.same_bool_if_present(label, message, "modelAdded", model_added)?;
                self.same_bool_if_present(label, message, "modelUpdated", model_updated)?;
                self.same_bool_if_present(label, message, "modelTracked", model_tracked)?;
                self.same_bool_if_present(label, message, "modelUntracked", model_untracked)
            }
            "camera_state_roundtrip" => {
                let state = self
                    .interface
                    .camera()
                    .get_camera_state(true)
                    .map_err(|err| format!("get_camera_state() failed: {err:?}"))?;
                let applied = self
                    .interface
                    .camera()
                    .set_camera_state(state, 0.0, 1.0, 1.0)
                    .map_err(|err| format!("set_camera_state() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "applied", applied)
            }
            _ => Err(format!("unsupported control call check `{label}`")),
        }
    }
}
