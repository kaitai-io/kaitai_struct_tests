local class = require("class")

MyCustomFx = class.class()

function MyCustomFx:_init(key, flag, some_bytes)
    if flag then
        self._key = key
    else
        self._key = -key
    end
end

function MyCustomFx:decode(data)
    local r = ""

    for i = 1, #data do
        local c = (data:byte(i) + self._key) % 0x100
        r = r .. string.char(c)
    end

    return r
end
