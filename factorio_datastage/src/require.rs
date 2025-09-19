use mlua::prelude::*;

use crate::VmData;

pub fn register_custom_require(vm: &Lua) -> LuaResult<()> {
    let g = vm.globals();
    g.raw_set("require", vm.create_function(require)?)?;

    let package = vm.create_table()?;
    package.raw_set("loaded", vm.create_table()?)?;
    g.raw_set("package", package)?;

    Ok(())
}

fn require(vm: &Lua, module: String) -> LuaResult<LuaValue> {
    if module.contains("..") || module.starts_with(['/', '\\']) {
        return Err(LuaError::runtime(format!(
            "explicit relative paths are not allowed! {module}"
        )));
    }

    let mut parts = if module.contains(['/', '\\']) {
        module.split(['/', '\\'])
    } else {
        module.split(['.', '.']) // its stupid but otherwise the return type of split would be different
    }
    .filter_map(|p| {
        if p.is_empty() {
            None
        } else {
            Some(p.to_string())
        }
    })
    .collect::<Vec<_>>();

    if parts.is_empty() {
        return Err(LuaError::runtime("empty module name"));
    }

    if parts
        .last()
        .ok_or_else(|| LuaError::runtime("empty module name (after already checking it?!)"))?
        .contains('.')
    {
        let last = parts.last_mut().ok_or_else(|| {
            LuaError::runtime("empty module name (after already checking it twice?!)")
        })?;
        let mut parts = last.split('.').collect::<Vec<_>>();
        parts.pop();
        *last = parts.join(".");
    }

    // actually load the file
    let mut file = None;
    let mut folder = None;
    let mut n_mod = None;
    let mut resolved_path = None;
    let data = vm
        .app_data_ref::<VmData>()
        .ok_or_else(|| LuaError::runtime("failed to get VM data"))?;
    for searcher in [
        require_absolute,
        require_relative,
        require_root,
        require_lualib,
    ] {
        if let Some((f, d, m, p)) = searcher(&parts, &data) {
            file = Some(f);
            folder = Some(d);
            n_mod = Some(m);
            resolved_path = Some(p);
            break;
        }
    }

    drop(data);
    let (Some(file), Some(folder), Some(n_mod), Some(resolved_path)) =
        (file, folder, n_mod, resolved_path)
    else {
        return Err(LuaError::runtime(format!("module not found: {module}")));
    };

    let loaded = vm
        .globals()
        .raw_get::<LuaTable>("package")?
        .raw_get::<LuaTable>("loaded")?;

    if loaded.contains_key(resolved_path.as_str())? {
        return loaded.raw_get(resolved_path);
    }

    let mut data = vm
        .app_data_mut::<VmData>()
        .ok_or_else(|| LuaError::runtime("failed to get mutable VM data"))?;
    let old_folder = data.current_folder.clone();
    let old_mod = data.current_mod.clone();
    data.current_folder = folder;
    data.current_mod = n_mod;
    drop(data);

    // skip utf8 BOM
    let start = if file.len() >= 3 && file[0] == 0xEF && file[1] == 0xBB && file[2] == 0xBF {
        3
    } else {
        0
    };

    let res = vm
        .load(&file[start..])
        .set_name(old_folder.clone() + "/" + &parts.join("/") + ".lua")
        .call::<LuaValue>(module)?;

    let mut data = vm
        .app_data_mut::<VmData>()
        .ok_or_else(|| LuaError::runtime("failed to get VM data"))?;
    data.current_folder = old_folder;
    data.current_mod = old_mod;
    drop(data);

    match res {
        LuaNil => loaded.set(resolved_path, true)?,
        _ => loaded.set(resolved_path, res.clone())?,
    }

    Ok(res)
}

fn require_absolute(parts: &[String], data: &VmData) -> Option<(Vec<u8>, String, String, String)> {
    if parts.len() < 2 {
        return None;
    }

    let first = &parts[0];
    if first.len() < 5 || !first.starts_with("__") || !first.ends_with("__") {
        return None;
    }

    let target_mod = &first[2..first.len() - 2];
    let path = parts[1..].join("/") + ".lua";

    let Some(m) = data.active_mods.get(target_mod) else {
        error!("[!!!!!] MODULE SPECIFIED ABSOLUTE PATH BUT MOD NOT FOUND: {target_mod} [{path}]");
        return None;
    };
    let Ok(file) = m.get_file(&path) else {
        error!("[!!!!!] MODULE SPECIFIED ABSOLUTE PATH BUT FILE NOT FOUND: {target_mod} [{path}]");
        return None;
    };

    Some((
        file,
        parts[1..parts.len() - 1].join("/"),
        target_mod.to_string(),
        format!("__{target_mod}__/{path}"),
    ))
}

fn require_relative(parts: &[String], data: &VmData) -> Option<(Vec<u8>, String, String, String)> {
    let folder = data.current_folder.clone();
    let relative = if parts.len() > 1 {
        parts[0..parts.len() - 1].join("/")
    } else {
        String::new()
    };

    let folder = if folder.is_empty() {
        relative
    } else if relative.is_empty() {
        folder
    } else {
        folder + "/" + &relative
    };

    let file_path = if folder.is_empty() {
        parts[parts.len() - 1].to_string() + ".lua"
    } else {
        folder.clone() + "/" + &parts[parts.len() - 1] + ".lua"
    };

    let file = data
        .active_mods
        .get(&data.current_mod)?
        .get_file(&file_path)
        .ok()?;

    Some((
        file,
        folder,
        data.current_mod.clone(),
        format!("__{}__/{file_path}", data.current_mod),
    ))
}

fn require_root(parts: &[String], data: &VmData) -> Option<(Vec<u8>, String, String, String)> {
    let folder = parts[0..parts.len() - 1].join("/");
    let file_path = parts.join("/") + ".lua";

    let file = data
        .active_mods
        .get(&data.current_mod)?
        .get_file(&file_path)
        .ok()?;

    Some((
        file,
        folder,
        data.current_mod.clone(),
        format!("__{}__/{file_path}", data.current_mod),
    ))
}

fn require_lualib(parts: &[String], data: &VmData) -> Option<(Vec<u8>, String, String, String)> {
    let folder = "lualib".to_string();
    let folder = if parts.len() > 1 {
        folder + "/" + &parts[0..parts.len() - 1].join("/")
    } else {
        folder
    };

    let file_path = folder.clone() + "/" + &parts[parts.len() - 1] + ".lua";
    let file = data.active_mods.get("core")?.get_file(&file_path).ok()?;

    Some((
        file,
        folder,
        "core".to_string(),
        format!("__core__/{file_path}"),
    ))
}
