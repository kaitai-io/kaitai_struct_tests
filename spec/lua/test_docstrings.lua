local luaunit = require("luaunit")

require("docstrings")

TestDocstrings = {}

function TestDocstrings:test_docstrings()
    local r = Docstrings:from_file("src/fixed_struct.bin")
end
