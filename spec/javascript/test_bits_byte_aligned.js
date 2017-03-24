var assert = require('assert');
var testHelper = require('testHelper');

testHelper('BitsByteAligned', 'src/fixed_struct.bin', function(r) {
    // 50 (6 + 2) = 010100|00
    assert.equal(r.one, 0b010100);
    // 41
    assert.equal(r.byte1, 0x41);
    // 43 (3 + 1 + 4) = 010|0|0011
    assert.equal(r.two, 0b010);
    assert.equal(r.three, false);
    // 4B
    assert.equal(r.byte2, 0x4b);
    // 2D 31 (14 + 2) = 00101101 001100|01
    assert.equal(r.four, 0b00101101001100);
    // FF
    assert.equal(r.byte3.toString(), new Uint8Array([0xff]).toString());
    // FF
    assert.equal(r.fullByte, 0xff);
    // 50
    assert.equal(r.byte4, 0x50);
});
