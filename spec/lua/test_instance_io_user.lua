local luaunit = require("luaunit")

require("instance_io_user")

TestInstanceIoUser = {}

function TestInstanceIoUser:test_instance_io_user()
    local r = InstanceIoUser:from_file("src/instance_io.bin")

    luaunit.assertEquals(r.qty_entries, 3)

    luaunit.assertEquals(r.entries[1].name, "the")
    luaunit.assertEquals(r.entries[2].name, "rainy")
    luaunit.assertEquals(r.entries[3].name, "day it is")
end
