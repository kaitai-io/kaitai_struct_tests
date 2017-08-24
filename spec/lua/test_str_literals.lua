local luaunit = require("luaunit")

require("str_literals")

TestStrLiterals = {}

function TestStrLiterals:test_str_literals()
    local r = StrLiterals:from_file("src/fixed_struct.bin")

    luaunit.assertEquals(self.str_to_arr(r.complex_str), {0, 1, 2, 7, 8, 10, 13, 9, 11, 12, 27, 61, 7, 10, 36, 226, 152, 187})
    luaunit.assertEquals(self.str_to_arr(r.double_quotes), {34, 34, 34})
    luaunit.assertEquals(self.str_to_arr(r.backslashes), {92, 92, 92})
    luaunit.assertEquals(self.str_to_arr(r.octal_eatup), {0, 50, 50})
    luaunit.assertEquals(self.str_to_arr(r.octal_eatup2), {2, 50})
end

function TestStrLiterals.str_to_arr(s)
    local arr = {}

    for i = 1, #s do
        arr[i] = s:byte(i)
    end

    return arr
end
