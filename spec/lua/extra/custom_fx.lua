local class = require("class")

Nested = {}
Nested.Deeply = {}
Nested.Deeply.CustomFx = class.class()

function Nested.Deeply.CustomFx:_init(foo)
end

function Nested.Deeply.CustomFx:decode(data)
    return "_" .. data .. "_"
end
