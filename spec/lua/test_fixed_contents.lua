require("fixed_contents")

TestFixedContents = {}

function TestFixedContents:test_fixed_contents()
    local r = FixedContents:from_file("src/fixed_struct.bin")
end
