"""Projection helpers for the canonical parity probe."""

from __future__ import annotations

import re

from . import core
from .types import records_field, rust_identifier, rust_semantic_path, semantic_path, simple_type, snake

def record_path(
    type_info: dict,
    desired: str,
    records: dict[str, dict],
    path: list[str | int] | None = None,
) -> tuple[dict, list[str | int]] | None:
    """Find a semantic record path for a flattened Lua result field."""
    if type_info.get("kind") != "record":
        return None
    record = records.get(type_info["name"])
    if record is None:
        return None

    if path:
        resolved = semantic_path(type_info, path, records)
        if resolved is not None:
            return resolved

    wanted = snake(desired)
    if wanted == "normalized_side_name":
        wanted = "side_name"
    if wanted in {"piece_name", "name_value"}:
        wanted = "name"
    if wanted in {"x_min", "z_min", "x_max", "z_max"}:
        wanted = {
            "x_min": "min_x",
            "z_min": "min_z",
            "x_max": "max_x",
            "z_max": "max_z",
        }[wanted]

    # Lua exposes Float2's second component as z, although the transport
    # record uses the conventional x/y names.
    if type_info["name"] == "Float2" and wanted == "z":
        wanted = "y"

    if type_info["name"] == "PieceMatrix" and re.fullmatch(r"m\d+", wanted):
        return semantic_path(type_info, ["m", int(wanted[1:])], records)

    direct = records_field(record, wanted)
    if direct is not None:
        return direct["type"], [direct["name"]]

    # Annotated native records may expose a Lua table's cardinality as a
    # separate scalar return (`optionCount`, `entryCount`, and similar).
    # Resolve that flattened count back to the list carrying the metadata so
    # both output selection and projection use the same semantic path.
    for field in record.get("fields", []):
        metadata = field.get("metadata", [])
        if field["type"].get("kind") in {"list", "fixed-array", "bytes"} and any(
            item == f"count-field:{desired}" for item in metadata
        ):
            return field["type"], [field["name"]]

    # Native records use nested Float2/Float3 values while Lua exposes their
    # components as fields such as frontX, posY, or targetZ.
    axis = wanted[-1:] if wanted and wanted[-1] in "xyzw" else ""
    base = wanted[:-1].rstrip("_") if axis else ""
    for field in record.get("fields", []):
        field_type = field["type"]
        if field_type.get("kind") != "record":
            continue
        variants = {
            snake(field["name"]),
            snake(field["name"]).replace("_direction", "").replace("_position", ""),
            snake(field["name"]).replace("direction", "dir").replace("position", "pos"),
        }
        base_variants = {variant.rstrip("_") for variant in variants}
        base_variants.update(
            variant.removesuffix("_position").removesuffix("_pos")
            for variant in list(base_variants)
        )
        base_variants.update(
            variant.removesuffix("_direction")
            .removesuffix("_dir")
            .removesuffix("_vector")
            .removesuffix("_vec")
            for variant in list(base_variants)
        )
        if type_info["name"] == "UnitWeaponVectors":
            if field["name"] == "weaponMuzzlePos":
                base_variants.update({"pos", "position"})
            elif field["name"] == "weaponDir":
                base_variants.update({"dir", "direction"})
        if axis and base in base_variants:
            nested_record = records.get(field_type["name"])
            nested = records_field(nested_record, axis) if nested_record is not None else None
            if nested is not None and simple_type(nested["type"]):
                return nested["type"], [field["name"], nested["name"]]

    # A flattened vector is frequently named by its semantic role (posX,
    # buildX, rayX) while the native record itself is simply Float3.
    if axis and base in {"normal", "pos", "position", "build", "hit", "ray", "vec"}:
        nested = records_field(record, axis)
        if nested is not None and simple_type(nested["type"]):
            return nested["type"], [nested["name"]]

    return None


def runtime_args(test: dict) -> list:
    runtime = test.get("lua_runtime") or {}
    if runtime.get("args") is not None:
        return runtime.get("args", [])
    if runtime.get("table") is not None:
        return [runtime.get("key")]
    return []


def output_index_for_field(
    outputs: list[dict],
    desired: str,
    records: dict[str, dict],
    explicit_index: int | None,
    return_index: int,
) -> int | None:
    """Find the semantic output containing one flattened Lua field."""
    wanted = snake(desired)
    aliases = {wanted}
    if wanted == "has_ai":
        # Spring.GetAIInfo exposes the presence flag as its first Lua return,
        # while the typed native result names the same value `isAI`.
        aliases.add("is_ai")
    if wanted == "has_custom_color":
        # Palette queries return the presence flag separately from the
        # sentinel custom index (`-1` means team color).
        aliases.add("using_custom_color")
    if wanted.endswith("_result"):
        aliases.add(wanted.removesuffix("_result"))
    if wanted == "feature_id":
        aliases.add("feature")
    aliases.update(
        {
            "hit_length" if wanted == "ray_length" else wanted,
            "hit_pos" if wanted.startswith("pos_") else wanted,
            "build_pos" if wanted.startswith("build_") else wanted,
        }
    )
    for index, output in enumerate(outputs):
        if snake(output["name"]) in aliases:
            return index
        if output["type"].get("kind") == "record":
            if record_path(output["type"], desired, records) is not None:
                return index
            # Flattened vector fields such as camPosX and dirZ identify the
            # containing output by the prefix before their final axis.
            flattened = snake(desired)
            axis = flattened[-1:] if flattened and flattened[-1] in "xyzw" else ""
            prefix = flattened[:-1].rstrip("_") if axis else ""
            if axis and prefix == snake(output["name"]):
                record_info = records.get(output["type"].get("name"), {})
                if records_field(record_info, axis) is not None:
                    return index
    if explicit_index is not None:
        return explicit_index - 1
    if 0 <= return_index < len(outputs):
        return return_index
    return None


def first_parameter_value(test: dict, *names: str) -> object | None:
    """Return the deterministic value used for a named parity parameter."""
    for name in names:
        parameter = test.get("params", {}).get(name)
        if parameter is None:
            continue
        if "wasm_fixed" in parameter:
            return parameter["wasm_fixed"]
        elif "fixed" in parameter:
            return parameter["fixed"]
        if "value" in parameter:
            return parameter["value"]
        values = parameter.get("values")
        if values:
            return values[0]
    return None


def projection_record_path(
    test: dict,
    field_name: str,
    path: list[str | int],
    type_info: dict,
    records: dict[str, dict],
) -> tuple[dict, list[str | int]] | None:
    """Apply the small number of Lua table aliases over native records."""
    record_name = type_info.get("name")
    desired = field_name
    resolved_path = list(path)

    if record_name == "RulesParamValue" and field_name == "value":
        declared_type = test.get("params", {}).get("value", {}).get("type")
        desired = {
            "bool": "boolValue",
            "f32": "floatValue",
            "float": "floatValue",
            "string": "stringValue",
        }.get(declared_type, desired)
    elif field_name == "value":
        if record_name == "UnitSensorRadius":
            desired = str(first_parameter_value(test, "sensorType") or "los")
        elif record_name == "UnitWeaponState":
            desired = str(first_parameter_value(test, "key") or "range")
        elif record_name == "UnitWeaponDamages":
            desired = str(first_parameter_value(test, "key") or "paralyzeDamageTime")
        elif record_name == "NumberOrBool":
            desired = "boolean" if first_parameter_value(test, "paramName") == "buildRange3D" else "number"
    elif record_name == "NumberOrBool" and field_name == "enabled":
        desired = "boolean"
    elif record_name == "UnitSensorRadius" and field_name == "radius":
        desired = str(first_parameter_value(test, "sensorType") or "los")
    elif record_name == "TeamResources" and field_name in {
        "currentLevel",
        "storage",
        "used",
        "produced",
        "excessed",
        "received",
        "sent",
    }:
        resource = str(first_parameter_value(test, "resource") or "metal")
        resource_prefix = "energy" if resource == "energy" else "metal"
        resource_fields = {
            "currentLevel": "Current",
            "storage": "Storage",
            "used": "Current",
            "produced": "Storage",
            "excessed": "Pull",
            "received": "Income",
            "sent": "Expense",
        }
        desired = resource_prefix + resource_fields[field_name]
    elif record_name == "ProjectileDamages" and field_name == "damageValue":
        tag = str(first_parameter_value(test, "tag") or "0")
        if tag == "0":
            resolved_path = ["damages", 0]
        else:
            desired = tag
    elif record_name == "PieceProjectileParams" and field_name in {
        "spinX",
        "spinY",
        "spinZ",
    }:
        resolved_path = ["spinVec", field_name[-1].lower()]
    elif record_name == "Float3" and field_name == "gravity":
        # GetProjectileGravity's Lua surface is scalar, whereas the common
        # native vector record keeps the value in its y component.
        resolved_path = ["y"]
    elif record_name == "WeaponDefInfo" and field_name == "damage":
        # WeaponDefs.damages[0] is the default armour damage, represented by
        # the compact native WeaponDefInfo record's `damage` field.
        desired = "damage"
        resolved_path = ["damage"]
    elif record_name == "FactoryQueueInfo" and field_name == "entryCount":
        resolved_path = ["counts"]
    elif record_name == "AllyTeamInfo" and field_name == "count":
        resolved_path = ["keys"]
    elif record_name == "SelectionCounts" and field_name == "defCount":
        # Lua returns the number of distinct definition keys as the second
        # result; the native record retains those keys explicitly.
        resolved_path = ["unitDefIDs"]
    elif record_name == "ProjectileTarget" and field_name in {
        "targetX",
        "targetY",
        "targetZ",
    }:
        # Spring.GetProjectileTarget returns target type, target ID, and a
        # three-component position as separate Lua results.  The native
        # record keeps the position nested in targetPos; the explicit mapping
        # also avoids interpreting Lua's one-based vector index as a field
        # index in the containing ProjectileTarget record.
        resolved_path = ["targetPos", field_name[-1].lower()]
    elif record_name == "WindData" and field_name == "windStrength":
        resolved_path = ["current"]
    elif record_name == "FeatureResurrect":
        if field_name == "unitDef":
            desired = "resurrectAs"
        elif field_name == "facing":
            desired = "facingDir"
    elif record_name == "UnitVectors":
        flattened = snake(field_name)
        axis = flattened[-1:] if flattened and flattened[-1] in "xyz" else ""
        prefix = flattened[:-1].rstrip("_") if axis else ""
        vector_name = {
            "front": "frontDir",
            "up": "upDir",
            "right": "rightDir",
        }.get(prefix)
        if vector_name is not None:
            resolved_path = [vector_name, axis]

    # Lua flattens named camera/direction vectors returned by input APIs into
    # fields such as camPosX and dirZ, while the semantic ABI keeps each
    # vector as a Float2/Float3 record.
    if record_name in {"Float2", "Float3"}:
        flattened = snake(field_name)
        axis = flattened[-1:] if flattened and flattened[-1] in "xyzw" else ""
        base = flattened[:-1].rstrip("_") if axis else ""
        if axis and base in {"cam_pos", "camera_pos", "dir", "direction", "position"}:
            resolved_path = [axis]

    return record_path(
        type_info,
        desired,
        records,
        resolved_path if resolved_path else None,
    )


def output_projection(
    test: dict,
    function: dict,
    records: dict[str, dict],
    functions: dict[tuple[str, str], dict] | None = None,
) -> list[dict] | None:
    """Map Lua return specs to typed result expressions.

    Each returned item contains a comparison field, its terminal semantic
    type, and a path relative to the generated WIT result value.  The Rust
    renderer turns the optional transform into the compact probe wire format.
    """
    runtime = test.get("lua_runtime") or {}
    if test.get("wasm_sequence") is not None:
        if functions is None:
            return None
        sequence = wasm_sequence_operations(test, functions, records)
        if sequence is None:
            return None
        return sequence_output_projection(test, sequence, records)
    returns = list(runtime.get("returns", test.get("wasm_returns", [])))
    for post_runtime in runtime.get("post", []):
        returns.extend(post_runtime.get("returns", []))
    if not returns:
        # A no-return native operation is still a useful parity row when the
        # Lua contract also has no return values (for example VFS.ScanAllDirs).
        if test.get("wasm_expected_error"):
            return []
        return [] if not test.get("compare", {}).get("fields") else None
    outputs = function["outputs"]
    projected = []
    for return_index, return_info in enumerate(returns):
        condition = None
        if isinstance(return_info, str):
            field_name = return_info
            transform = None
            path = []
            explicit_index = None
        elif isinstance(return_info, dict):
            field_name = return_info.get("field")
            transform = return_info.get("transform")
            path = return_info.get("path", [])
            explicit_index = return_info.get("index")
            condition = return_info.get("when")
        else:
            return None
        if not field_name:
            return None
        # Lua may expose helper fields needed by the native parity checker
        # that are not part of the native result being projected.  Emit only
        # the canonical comparison surface on the compact probe wire format.
        if field_name not in test.get("compare", {}).get("fields", []):
            continue

        if len(outputs) == 1:
            output_index = 0
            output = outputs[output_index]
            expression = "value"
        else:
            # The WIT renderer represents multi-result native records as one
            # record even though the semantic model stores their fields as
            # separate outputs.  Prefer the named/path field over the Lua
            # return index; Lua and native result ordering are not guaranteed
            # to be identical (for example FindUnitCmdDesc).
            target_name = path[0] if path and isinstance(path[0], str) else field_name
            output_index = output_index_for_field(
                outputs,
                target_name,
                records,
                explicit_index,
                return_index,
            )
            if output_index is None and target_name != field_name:
                # A Lua path can name a table key that is represented by a
                # differently named compact native field (WeaponDefInfo's
                # `damages[0]` is the canonical example).
                output_index = output_index_for_field(
                    outputs,
                    field_name,
                    records,
                    explicit_index,
                    return_index,
                )
            if output_index is None:
                return None
            if output_index < 0 or output_index >= len(outputs):
                return None
            output = outputs[output_index]
            expression = f"value.{rust_identifier(output['name'])}"
        type_info = output["type"]
        optional = False

        # Some Lua APIs return several scalar values while the native
        # semantic model represents the same values as one list. Project the
        # requested positional element instead of serializing the whole list
        # once for every flattened Lua field.
        if (
            len(outputs) == 1
            and type_info.get("kind") in {"list", "fixed-array"}
            and len(returns) > 1
            and not path
            and explicit_index is None
        ):
            type_info = type_info["element"]
            expression = f"{expression}.get({return_index}).copied().unwrap_or_default()"

        if type_info.get("kind") == "record":
            resolved = projection_record_path(
                test,
                field_name,
                path,
                type_info,
                records,
            )
            if resolved is None:
                return None
            type_info, semantic_path = resolved
            expression = rust_semantic_path(expression, semantic_path)
            if transform in {"string_len", "table_count"} and type_info.get("kind") in {
                "list",
                "fixed-array",
                "bytes",
            }:
                type_info = {"kind": "scalar", "name": "u32"}
                expression = f"{expression}.len() as u32"
        elif type_info.get("kind") == "option":
            if transform == "truthy":
                type_info = {"kind": "scalar", "name": "bool"}
                expression = f"{expression}.is_some()"
            else:
                inner = type_info.get("inner", type_info.get("element"))
                if inner is None or inner.get("kind") != "record":
                    return None
                resolved = projection_record_path(test, field_name, path, inner, records)
                if resolved is None:
                    return None
                type_info, semantic_path = resolved
                field_expression = rust_semantic_path("value", semantic_path)
                expression = f"{expression}.as_ref().map(|value| {field_expression})"
                optional = True
        elif type_info.get("kind") in {"list", "fixed-array", "bytes"}:
            if test.get("id") == "get_player_traffic_missing_packet" and type_info.get("kind") == "list":
                element = type_info.get("element", {})
                if element.get("kind") != "record":
                    return None
                type_info = {"kind": "scalar", "name": "i32"}
                expression = (
                    f"{expression}.first().map(|value| if value.bytes_sent >= u32::MAX "
                    f"{{ -1 }} else {{ value.bytes_sent as i32 }}).unwrap_or(-1)"
                )
            elif (
                test.get("id") in {"get_unit_transform_matrix", "get_feature_transform_matrix"}
                and type_info.get("kind") in {"list", "fixed-array"}
                and field_name.startswith("m")
                and field_name[1:].isdigit()
            ):
                matrix_index = int(field_name[1:]) - 1
                if not 0 <= matrix_index < 16:
                    return None
                type_info = type_info["element"]
                expression = (
                    f"{expression}.get({matrix_index}).copied().unwrap_or_default()"
                )
            elif test.get("id") == "get_frustum_planes" and len(path) == 2:
                plane_offsets = {
                    "topFrustumPlane": 0,
                    "botFrustumPlane": 4,
                    "lftFrustumPlane": 8,
                    "rgtFrustumPlane": 12,
                }
                offset = plane_offsets.get(path[0])
                component = path[1]
                if offset is None or not isinstance(component, int) or not 1 <= component <= 3:
                    return None
                type_info = {"kind": "scalar", "name": "f32"}
                expression = (
                    f"{expression}.get({offset + component - 1}).copied().unwrap_or_default()"
                )
            elif transform in {"string_len", "table_count"}:
                type_info = {"kind": "scalar", "name": "u32"}
                expression = f"{expression}.len() as u32"
            elif transform == "string_hex":
                if type_info.get("kind") == "bytes" or (
                    type_info.get("kind") in {"list", "fixed-array"}
                    and type_info["element"].get("kind") == "scalar"
                    and type_info["element"].get("name") == "u8"
                ):
                    type_info = {"kind": "hex-bytes"}
                else:
                    return None
            elif type_info.get("kind") in {"list", "fixed-array"}:
                element = type_info["element"]
                if not simple_type(element):
                    if (
                        element.get("kind") == "record"
                        and field_name in {"count", "entryCount"}
                    ):
                        type_info = {"kind": "scalar", "name": "u32"}
                        expression = f"{expression}.len() as u32"
                    elif element.get("kind") != "record":
                        return None
                if transform not in {
                    None,
                    "flat_float3_list",
                    "table_keys",
                    "table_int_keys",
                    "table_int_values",
                    "table_values",
                    "table_string_int_pairs",
                    "unit_def_counts",
                    "unit_def_unit_groups",
                    "start_positions",
                    "nested_unit_ids",
                }:
                    return None
            elif type_info.get("kind") == "bytes":
                if transform not in {None, "table_values"}:
                    return None
        elif transform == "table_count":
            # Some Lua tables are represented by a native count scalar.  The
            # semantic value is already the desired count; do not call len()
            # on it.
            if type_info.get("kind") != "scalar":
                return None
            type_info = {"kind": "scalar", "name": "u32"}
        elif transform == "build_status_can_build":
            # Utils.TestBuildOrder carries both the legacy numeric status and
            # its already-normalized boolean in the native result.  Prefer
            # the typed boolean when it is present; retain the transform as
            # documentation of the Lua status rule.
            if type_info.get("kind") == "scalar" and type_info.get("name") == "bool":
                pass
            elif type_info.get("kind") == "scalar":
                source_type = type_info["name"]
                type_info = {"kind": "scalar", "name": "bool"}
                expression = f"({expression} >= 2 as {source_type})"
            else:
                return None
        elif transform == "table_nonempty":
            if type_info.get("kind") == "scalar" and type_info.get("name") == "bool":
                pass
            else:
                return None
        elif transform == "truthy":
            source_type = type_info
            type_info = {"kind": "scalar", "name": "bool"}
            if source_type.get("kind") == "scalar" and source_type.get("name") == "bool":
                pass
            elif source_type.get("kind") == "string":
                expression = f"(!{expression}.is_empty())"
            elif source_type.get("kind") == "scalar":
                expression = f"({expression} != 0 as {source_type['name']})"
            else:
                return None
        elif transform == "valid_id":
            type_info = {"kind": "scalar", "name": "bool"}
            expression = f"({expression} >= 0)"
        elif transform == "string_len":
            source_kind = type_info.get("kind")
            type_info = {"kind": "scalar", "name": "u32"}
            if source_kind != "scalar":
                expression = f"{expression}.len() as u32"
        elif transform in {"nil_to_minus_one", "false_to_minus_one"}:
            type_info = {"kind": "scalar", "name": "i32"}
            expression = expression
        elif transform == "return_count":
            # A control-only Lua call has no return values, while a
            # setter/getter row uses this transform for the getter's Lua
            # arity.  The native model keeps the getter's individual outputs,
            # so its output count is the independent source for the latter.
            type_info = {"kind": "scalar", "name": "u32"}
            setter = runtime.get("set")
            setter_call = None
            if isinstance(setter, dict):
                setter_call = setter.get("call")
            elif isinstance(setter, list) and len(setter) == 1 and isinstance(setter[0], dict):
                setter_call = setter[0].get("call")
            getter_call = runtime.get("call")
            getter_has_distinct_call = (
                setter_call is not None
                and getter_call is not None
                and setter_call != getter_call
            )
            expression = f"{len(outputs)}u32" if getter_has_distinct_call else "0u32"
        elif transform not in {None, "return_count"}:
            return None

        projected.append(
            {
                "field": field_name,
                "type": type_info,
                "expression": expression,
                "transform": transform,
                "optional": optional,
                "condition": condition,
            }
        )
    return projected


# These operations are defined by the rendering module in core. They are
# resolved after core has finished loading, so this module remains a pure
# semantic helper while the historical call sites keep their names.
wasm_sequence_operations = core.wasm_sequence_operations
sequence_output_projection = core.sequence_output_projection
