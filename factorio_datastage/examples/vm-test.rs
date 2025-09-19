#![allow(clippy::unwrap_used, unsafe_code)]

use mlua::Lua;

fn main() {
    println!("Safe VM:");
    let lua_vm = Lua::new();
    lua_vm
        .load("for name, _ in pairs(debug) do print(\"debug.\" .. name) end")
        .exec()
        .unwrap();

    println!("\nUnsafe VM:");
    let lua_vm = unsafe { Lua::unsafe_new() };
    lua_vm
        .load("for name, _ in pairs(debug) do print(\"debug.\" .. name) end")
        .exec()
        .unwrap();
}
