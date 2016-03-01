var assert = require('assert');
var testHelper = require('testHelper');

testHelper('BufferedStruct', 'src/buffered_struct.bin', function(r) {
    assert.equal(r.len1, 0x10);
    assert.equal(r.block1.number1, 0x42);
    assert.equal(r.block1.number2, 0x43);
    assert.equal(r.len2, 0x8);
    assert.equal(r.block2.number1, 0x44);
    assert.equal(r.block2.number2, 0x45);
    assert.equal(r.finisher, 0xee);
});
