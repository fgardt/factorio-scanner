#![allow(clippy::unnecessary_wraps)]

use std::{
    collections::HashMap,
    io::{Read as _, Write as _},
};

use base64::{engine::general_purpose, Engine};
use flate2::{read::ZlibDecoder, write::ZlibEncoder};
use mlua::prelude::*;
use serde::Serialize;

use mod_util::mod_info::Version;
use types::MathExpression;

pub fn register_lua_helpers(vm: &Lua) -> LuaResult<()> {
    let helpers = vm.create_table()?;
    helpers.raw_set("table_to_json", vm.create_function(table_to_json)?)?;
    helpers.raw_set("json_to_table", vm.create_function(json_to_table)?)?;
    helpers.raw_set("write_file", vm.create_function(write_file)?)?;
    helpers.raw_set("send_udp", vm.create_function(send_udp)?)?;
    // recv_udp, not available in setting/data stage
    helpers.raw_set("remove_path", vm.create_function(remove_path)?)?;
    helpers.raw_set(
        "direction_to_string",
        vm.create_function(direction_to_string)?,
    )?;
    helpers.raw_set(
        "evaluate_expression",
        vm.create_function(evaluate_expression)?,
    )?;
    helpers.raw_set("encode_string", vm.create_function(encode_string)?)?;
    helpers.raw_set("decode_string", vm.create_function(decode_string)?)?;
    // parse_map_exchange_string, not available in setting/data stage
    // is_valid_sound_path, not available in setting/data stage
    // is_valid_sprite_path, not available in setting/data stage
    // create_profiler, not available in setting/data stage
    helpers.raw_set("compare_versions", vm.create_function(compare_versions)?)?;

    helpers.raw_set("game_version", env!("CARGO_PKG_VERSION_PRE"))?;

    let g = vm.globals();
    g.raw_set("helpers", helpers)?;

    Ok(())
}

#[allow(clippy::needless_pass_by_value)]
fn table_to_json(_vm: &Lua, data: LuaTable) -> LuaResult<String> {
    let json = serde_json::to_string(&data).map_err(|e| LuaError::RuntimeError(e.to_string()))?;
    Ok(json)
}

#[allow(clippy::needless_pass_by_value)]
fn json_to_table(vm: &Lua, json: String) -> LuaResult<LuaValue> {
    let json = serde_json::from_str::<serde_json::Value>(&json)
        .map_err(|e| LuaError::RuntimeError(e.to_string()))?;

    let opts = LuaSerializeOptions::new().detect_serde_json_arbitrary_precision(true);
    let serializer = mlua::serde::Serializer::new_with_options(vm, opts);

    json.serialize(serializer)
        .map_err(|e| LuaError::RuntimeError(e.to_string()))
}

fn write_file(
    _vm: &Lua,
    (_filename, _data, _append): (String, LuaValue, Option<bool>),
) -> LuaResult<()> {
    // noop

    Ok(())
}

fn send_udp(_vm: &Lua, (_port, _data): (u16, LuaValue)) -> LuaResult<()> {
    // noop

    Ok(())
}

fn remove_path(_vm: &Lua, _path: String) -> LuaResult<()> {
    // noop

    Ok(())
}

fn direction_to_string(_vm: &Lua, direction: u8) -> LuaResult<String> {
    let res = match direction {
        0 => "North",
        1 => "NorthNorthEast",
        2 => "NorthEast",
        3 => "EastNorthEast",
        4 => "East",
        5 => "EastSouthEast",
        6 => "SouthEast",
        7 => "SouthSouthEast",
        8 => "South",
        9 => "SouthSouthWest",
        10 => "SouthWest",
        11 => "WestSouthWest",
        12 => "West",
        13 => "WestNorthWest",
        14 => "NorthWest",
        15 => "NorthNorthWest",
        _ => {
            return Err(LuaError::RuntimeError(format!(
                "Invalid direction: {direction}"
            )))
        }
    };

    Ok(res.to_owned())
}

fn evaluate_expression(
    _vm: &Lua,
    (expression, variables): (String, Option<HashMap<String, f64>>),
) -> LuaResult<f64> {
    let vars = variables.unwrap_or_default();

    MathExpression(expression)
        .eval(&vars)
        .map_err(|e| LuaError::RuntimeError(e.to_string()))
}

#[allow(clippy::needless_pass_by_value)]
fn encode_string(_vm: &Lua, string: String) -> LuaResult<Option<String>> {
    let mut deflate = ZlibEncoder::new(Vec::new(), flate2::Compression::new(9));
    if deflate.write_all(string.as_bytes()).is_err() {
        return Ok(None);
    }
    let Ok(compressed) = deflate.finish() else {
        return Ok(None);
    };

    let encoded = general_purpose::STANDARD.encode(compressed);

    Ok(Some(encoded))
}

#[allow(clippy::needless_pass_by_value)]
fn decode_string(_vm: &Lua, string: String) -> LuaResult<Option<String>> {
    let Ok(compressed) = general_purpose::STANDARD.decode(string.as_str()) else {
        return Ok(None);
    };

    let mut inflate = ZlibDecoder::new(compressed.as_slice());
    let mut uncompressed = String::new();

    if inflate.read_to_string(&mut uncompressed).is_err() {
        return Ok(None);
    }

    Ok(Some(uncompressed))
}

fn compare_versions(_vm: &Lua, (first, second): (String, String)) -> LuaResult<i32> {
    fn invalid_version(input: &str) -> LuaError {
        LuaError::RuntimeError(format!(
            "Invalid version: Expected 'a.b' or 'a.b.c' but '{input}' was given"
        ))
    }

    let Ok(a) = first.parse::<Version>() else {
        return Err(invalid_version(&first));
    };
    let Ok(b) = second.parse::<Version>() else {
        return Err(invalid_version(&second));
    };

    Ok(a.cmp(&b) as i32)
}
