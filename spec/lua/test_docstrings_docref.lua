local luaunit = require("luaunit")

require("docstrings_docref")

TestDocstringsDocref = {}

function TestDocstringsDocref:test_docstrings_docref()
    local r = DocstringsDocref:from_file("src/fixed_struct.bin")
end
