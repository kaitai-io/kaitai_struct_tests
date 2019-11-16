local luaunit = require("luaunit")

require("user_type")

TestUserType = {}

function TestUserType:test_user_type()
    local r = UserType:from_file("src/repeat_until_s4.bin")

    luaunit.assertEquals(r.one.width, 0x42)
    luaunit.assertEquals(r.one.height, 0x1337)
end
