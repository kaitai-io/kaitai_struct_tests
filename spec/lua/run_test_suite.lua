#!/usr/bin/env lua

local luaunit = require("luaunit")
local lfs = require("lfs")

function srequire(t)
    local success, err = xpcall(function() require(t) end, require_err_handler(t))
    return success
end

function require_err_handler(t)
    return function(err)
        local first = t:sub(1, 1)
        local rest = t:sub(2)
        local class_name = first:upper() .. rest:gsub("_(%w)", function(c) return c:upper() end)
        _G[class_name] = {}
        _G[class_name][t] = function()
            luaunit.fail(err)
        end
    end
end

local spec_path = lfs.currentdir() .. "/spec/lua/"
local test_prefix = "test_"
local test_suffix = ".lua"
for file, dir in lfs.dir(spec_path) do
    if lfs.attributes(spec_path .. file, "mode") == "file" and file:sub(-#test_suffix) == test_suffix and file:sub(1, #test_prefix) == test_prefix then
        srequire(file:sub(1, -#test_suffix - 1))
    end
end

-- Execute the test cases
local runner = luaunit.LuaUnit.new()
os.exit(runner:runSuite())
