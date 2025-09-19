--[[ require("util")

-- https://web.archive.org/web/20131225070434/http://snippets.luacode.org/snippets/Deep_Comparison_of_Two_Values_3
local function deepcompare(t1, t2, ignore_mt)
    local ty1 = type(t1)
    local ty2 = type(t2)
    if ty1 ~= ty2 then return false end
    -- non-table types can be directly compared
    if ty1 ~= 'table' and ty2 ~= 'table' then return t1 == t2 end
    -- as well as tables which have the metamethod __eq
    local mt = getmetatable(t1)
    if not ignore_mt and mt and mt.__eq then return t1 == t2 end
    for k1, v1 in pairs(t1) do
        local v2 = t2[k1]
        if v2 == nil or not deepcompare(v1, v2) then return false end
    end
    for k2, v2 in pairs(t2) do
        local v1 = t1[k2]
        if v1 == nil or not deepcompare(v1, v2) then return false end
    end
    return true
end

-- https://stackoverflow.com/a/54140176
local function do_tables_match(a, b)
    if type(a) ~= "table" or type(b) ~= "table" then
        return false
    end
    return table.concat(a) == table.concat(b)
end

local function serpent_compare(a, b)
    return serpent.dump(a) == serpent.dump(b)
end
]]


local added = {}
local removed = {}
local changed = {}

local raw = data.raw
local protos = defines.prototypes

for groupname, group in pairs(protos) do
    for typename, _ in pairs(group) do
        local old = MODNAME_RESOLVER_OLD_RAW[typename] or {}
        local new = raw[typename] or {}
        local checked = {}

        for name, proto in pairs(new) do
            checked[name] = true
            local old_proto = old[name]

            if not old_proto then
                added[groupname] = added[groupname] or {}
                added[groupname][typename] = added[groupname][typename] or {}
                table.insert(added[groupname][typename], name)
                goto continue
            end

            -- if not table_compare(proto, old_proto) then
            --     changed[groupname] = changed[groupname] or {}
            --     changed[groupname][typename] = changed[groupname][typename] or {}
            --     table.insert(changed[groupname][typename], name)
            -- end

            ::continue::
        end

        for name, proto in pairs(old) do
            if checked[name] then goto continue end

            removed[groupname] = removed[groupname] or {}
            removed[groupname][typename] = removed[groupname][typename] or {}
            table.insert(removed[groupname][typename], name)

            ::continue::
        end
    end
end

MODNAME_RESOLVER_OLD_RAW = table.deepcopy(data.raw)

return {
    added = added,
    removed = removed,
    changed = changed,
}
