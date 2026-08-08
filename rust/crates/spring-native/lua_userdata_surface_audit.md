# Lua userdata / class surface audit

Generated from the active registration sites in `rts/Lua/LuaVAO.cpp`, `LuaVBO.cpp`, `LuaFonts.cpp`, `LuaRBOs.cpp`, and `LuaFBOs.cpp` plus their implementation documentation.
This is separate from the free-callout inventory in `lua_functions.md`; native modules use explicit integer handles where Lua uses userdata.

## Summary

- Inventory rows: 65
- Matched rows: 60
- Matched but untested: 0
- Unclassified gaps: 0
- Native ABI missing for declared counterpart: 0

A complete parity claim requires zero unclassified gaps and zero matched-but-untested rows. `by-design` lifecycle rows are explicit exceptions, not coverage omissions.

## Inventory

| Surface | Kind | Lua member | Lua signature | Native counterpart | Status |
| --- | --- | --- | --- | --- | --- |
| `VAO` | `method` | `AddFeatureDefsToSubmission` | featureDefIDs: number|number[] -> number submittedCount | `Gfx.add_feature_defs_to_submission_vao` | **matched** |
| `VAO` | `method` | `AddFeaturesToSubmission` | featureIDs: number|number[] -> number submittedCount | `Gfx.add_features_to_submission_vao` | **matched** |
| `VAO` | `method` | `AddUnitDefsToSubmission` | unitDefIDs: number|number[] -> number submittedCount | `Gfx.add_unit_defs_to_submission_vao` | **matched** |
| `VAO` | `method` | `AddUnitsToSubmission` | unitIDs: number|number[] -> number submittedCount | `Gfx.add_units_to_submission_vao` | **matched** |
| `VAO` | `method` | `AttachIndexBuffer` | vbo: VBO -> nil | `Gfx.attach_index_buffer_vao` | **matched** |
| `VAO` | `method` | `AttachInstanceBuffer` | vbo: VBO -> nil | `Gfx.attach_instance_buffer_vao` | **matched** |
| `VAO` | `method` | `AttachVertexBuffer` | vbo: VBO -> nil | `Gfx.attach_vertex_buffer_vao` | **matched** |
| `VAO` | `method` | `ClearSubmission` | undocumented | `Gfx.clear_submission_vao` | **matched** |
| `VAO` | `method` | `Delete` |  -> nil | `Gfx.delete_vao` | **matched** |
| `VAO` | `method` | `DrawArrays` | glEnum: number primitivesMode, vertexCount: number?, vertexFirst: number?, instanceCount: number?, instanceFirst: number? -> nil | `Gfx.draw_arrays_vao` | **matched** |
| `VAO` | `method` | `DrawElements` | glEnum: number primitivesMode, drawCount: number?, baseIndex: number?, instanceCount: number?, baseVertex: number?, baseInstance: number? -> nil | `Gfx.draw_elements_vao` | **matched** |
| `VAO` | `method` | `RemoveFromSubmission` | index: number -> nil | `Gfx.remove_from_submission_vao` | **matched** |
| `VAO` | `method` | `Submit` |  -> nil | `Gfx.submit_vao` | **matched** |
| `VAO` | `lifecycle` | `__gc` | metatable finalizer | — | **by-design** |
| `VBO` | `method` | `BindBufferRange` | index: integer should be in the range between, elementOffset: integer?, elementCount: number?, target: number? glEnum -> integer bindingIndex when successful, -1 otherwise | `Gfx.bind_buffer_range_vbo` | **matched** |
| `VBO` | `method` | `Clear` | undocumented | `Gfx.clear_vbo` | **matched** |
| `VBO` | `method` | `CopyTo` | destVBO: VBO, copySizeInBytes: integer -> boolean success | `Gfx.copy_to_vbo` | **matched** |
| `VBO` | `method` | `Define` | size: number The maximum number of elements this VBO can have., attribs: number|VBOAttributeDef[] -> nil | `Gfx.define_vbo` | **matched** |
| `VBO` | `method` | `Delete` |  -> nil | `Gfx.delete_vbo` | **matched** |
| `VBO` | `method` | `Download` | attributeIndex: integer? (Default: `-1`) when supplied with non-default value: only data, elementOffset: integer? (Default: `0`) download data starting from this element, elementCount: number? number of elements to download, forceGPURead: boolean? (Default: `false`) force downloading the data from GPU buffer as opposed -> number[] vboData | `Gfx.download_vbo` | **matched** |
| `VBO` | `method` | `DumpDefinition` |  -> nil | `Gfx.dump_definition_vbo` | **matched** |
| `VBO` | `method` | `GetBufferSize` |  -> number elementsCount; number bufferSizeInBytes; number size | `Gfx.get_vboinfo` | **matched** |
| `VBO` | `method` | `GetID` |  -> integer bufferID | `Gfx.get_idvbo` | **matched** |
| `VBO` | `method` | `InstanceDataFromFeatureDefIDs` | featureDefIDs: number|number[], attrID: integer, teamIdOpt: integer?, elementOffset: integer? -> [number,number,number,number] instanceData; integer elementOffset; integer attrID | `Gfx.instance_data_from_feature_defs_vbo` | **matched** |
| `VBO` | `method` | `InstanceDataFromFeatureIDs` | featureIDs: number|number[], attrID: integer, teamIdOpt: integer?, elementOffset: integer? -> [number,number,number,number] instanceData; integer elementOffset; integer attrID | `Gfx.instance_data_from_features_vbo` | **matched** |
| `VBO` | `method` | `InstanceDataFromUnitDefIDs` | unitDefIDs: number|number[], attrID: integer, teamIdOpt: integer?, elementOffset: integer? -> [number,number,number,number] instanceData; integer elementOffset; integer attrID | `Gfx.instance_data_from_unit_defs_vbo` | **matched** |
| `VBO` | `method` | `InstanceDataFromUnitIDs` | unitIDs: number|number[], attrID: integer, teamIdOpt: integer?, elementOffset: integer? -> [number,number,number,number] instanceData; integer elementOffset; integer attrID | `Gfx.instance_data_from_units_vbo` | **matched** |
| `VBO` | `method` | `MatrixDataFromProjectileIDs` | projectileIDs: integer|integer[], attrID: integer, teamIdOpt: integer?, elementOffset: integer? -> number[] matDataVec 4x4 matrix; integer elemOffset; integer|[integer,integer,integer,integer] attrID | `Gfx.matrix_data_from_projectiles_vbo` | **matched** |
| `VBO` | `method` | `ModelsVBO` |  -> nil|number buffer size in bytes | `Gfx.models_vbo` | **matched** |
| `VBO` | `method` | `UnbindBufferRange` | index: integer, elementOffset: integer?, elementCount: number?, target: number? glEnum -> number bindingIndex when successful, -1 otherwise | `Gfx.unbind_buffer_range_vbo` | **matched** |
| `VBO` | `method` | `Upload` | vboData: number[] Array of values to upload into the VBO., attributeIndex: integer? (Default: `-1`), elemOffset: integer? (Default: `0`) The index in destination VBO (on GPU) at which storing begins., luaStartIndex: integer? (Default: `1`) The index of `vboData` at which copying begins., luaFinishIndex: integer? (Default: `#vboData`) The index of `vboData` at which copying ends. -> number[] indexData; integer elemOffset; integer|[integer,integer,integer,integer] attrID | `Gfx.upload_vbo` | **matched** |
| `VBO` | `lifecycle` | `__gc` | metatable finalizer | — | **by-design** |
| `LuaFont` | `method` | `Begin` | userDefinedBlending: boolean? When `true` doesn't set the gl.BlendFunc automatically. Defaults to `false`. | `Gfx.font_begin` | **matched** |
| `LuaFont` | `method` | `BindTexture` | undocumented | `Gfx.font_bind_texture` | **matched** |
| `LuaFont` | `method` | `End` | no documented parameters | `Gfx.font_end` | **matched** |
| `LuaFont` | `method` | `GetTextHeight` | text: string -> number height; number descender; number lines | `Gfx.font_get_text_height` | **matched** |
| `LuaFont` | `method` | `GetTextWidth` | text: string -> number width | `Gfx.font_get_text_width` | **matched** |
| `LuaFont` | `method` | `Print` | text: string, x: number, y: number, size: number? Defaults to the font's point size., options: string? Flag characters for alignment, outline, shadow, scaling, etc. (e.g. `"co"` for center and outline). | `Gfx.font_print` | **matched** |
| `LuaFont` | `method` | `PrintWorld` | text: string, x: number, y: number, z: number, size: number? Defaults to the font's point size., options: string? Flag characters for alignment, outline, shadow, scaling, etc. (e.g. `"co"` for center and outline). | `Gfx.font_print_world` | **matched** |
| `LuaFont` | `method` | `SetAutoOutlineColor` | enabled: boolean | `Gfx.font_set_auto_outline_color` | **matched** |
| `LuaFont` | `method` | `SetOutlineColor` | color: table Four-component RGBA array (`{r, g, b, a}`), or pass `r`, `g`, `b`, and optional `a` as separate numbers (requires at least three numeric components after the font). | `Gfx.font_set_outline_color` | **matched** |
| `LuaFont` | `method` | `SetTextColor` | color: table Four-component RGBA array (`{r, g, b, a}`), or pass `r`, `g`, `b`, and optional `a` as separate numbers (requires at least three numeric components after the font). | `Gfx.font_set_text_color` | **matched** |
| `LuaFont` | `method` | `SubmitBuffered` | noBillboarding: boolean? When `false` sets 3d billboard mode. Defaults to `true`., userDefinedBlending: boolean? When `true` doesn't set the gl.BlendFunc automatically. Defaults to `false`. | `Gfx.font_submit_buffered` | **matched** |
| `LuaFont` | `method` | `WrapText` | text: string, maxWidth: number, maxHeight: number? Defaults to an engine-defined maximum height., size: number? Defaults to the font's point size. -> string wrappedText; number lineCount | `Gfx.font_wrap_text` | **matched** |
| `LuaFont` | `property` | `descender` | property read | `Gfx.get_font_info.descender` | **matched** |
| `LuaFont` | `property` | `family` | property read | `Gfx.get_font_info.family` | **matched** |
| `LuaFont` | `property` | `height` | property read | `Gfx.get_font_info.line_height` | **matched** |
| `LuaFont` | `property` | `lineheight` | property read | `Gfx.get_font_info.line_height` | **matched** |
| `LuaFont` | `property` | `outlineweight` | property read | `Gfx.get_font_info.outline_weight` | **matched** |
| `LuaFont` | `property` | `outlinewidth` | property read | `Gfx.get_font_info.outline_width` | **matched** |
| `LuaFont` | `property` | `path` | property read | `Gfx.get_font_info.path` | **matched** |
| `LuaFont` | `property` | `size` | property read | `Gfx.get_font_info.size` | **matched** |
| `LuaFont` | `property` | `style` | property read | `Gfx.get_font_info.style` | **matched** |
| `LuaFont` | `property` | `textureheight` | property read | `Gfx.get_font_info.texture_height` | **matched** |
| `LuaFont` | `property` | `texturewidth` | property read | `Gfx.get_font_info.texture_width` | **matched** |
| `LuaFont` | `lifecycle` | `__gc` | metatable finalizer | — | **by-design** |
| `RBO` | `property` | `format` | property read | `Gfx.get_rboinfo.format` | **matched** |
| `RBO` | `property` | `samples` | property read | `Gfx.get_rboinfo.samples` | **matched** |
| `RBO` | `property` | `target` | property read | `Gfx.get_rboinfo.target` | **matched** |
| `RBO` | `property` | `valid` | property read | `Gfx.get_rboinfo.valid` | **matched** |
| `RBO` | `property` | `xsize` | property read | `Gfx.get_rboinfo.xsize` | **matched** |
| `RBO` | `property` | `ysize` | property read | `Gfx.get_rboinfo.ysize` | **matched** |
| `RBO` | `lifecycle` | `__gc` | metatable finalizer | — | **by-design** |
| `FBO` | `property` | `dynamic attachment keys` | property read | `Gfx.set_fboattachment`, `Gfx.set_fbodraw_buffers`, `Gfx.set_fboread_buffer` | **matched** |
| `FBO` | `lifecycle` | `__gc` | metatable finalizer | — | **by-design** |

## Explicit design boundaries

- `FBO.__gc` — Native modules own explicit integer handles and call DeleteFBO; Rust has no Lua garbage collector boundary.
- `LuaFont.__gc` — Native modules own explicit integer font handles and call DeleteFont; Rust has no Lua garbage collector boundary.
- `RBO.__gc` — Native modules own explicit integer handles and call DeleteRBO; Rust has no Lua garbage collector boundary.
- `VAO.__gc` — Native modules own explicit integer handles and call DeleteVAO; Rust has no Lua garbage collector boundary.
- `VBO.__gc` — Native modules own explicit integer handles and call DeleteVBO; Rust has no Lua garbage collector boundary.

## Audit interpretation

- A Lua userdata method with no native counterpart is a porting gap, not a harmless naming mismatch.
- A native integer-handle API is an acceptable representation change only when its lifecycle, result values, and documented parameters are tested against the Lua object.
- Dynamic FBO fields are deliberately listed as a gap until typed native attachment operations and readback semantics exist.
