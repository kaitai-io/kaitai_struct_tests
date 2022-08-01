local class = require("class")

CustomFxNoArgs = class.class()

function CustomFxNoArgs:_init(foo)
end

function CustomFxNoArgs:decode(data)
    return "_" .. data .. "_"
end
