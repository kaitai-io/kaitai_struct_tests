var assert = require('assert');
var testHelper = require('testHelper');

testHelper('BitsSimple', 'src/fixed_struct.bin', function(r) {
    assert.equal(r.byte1, 0x50);
    assert.equal(r.byte2, 0x41);

    // 43 (1 + 3 + 4) = 0|100|0011
    assert.strictEqual(r.bitsA, false);
    assert.equal(r.bitsB, 0b100);
    assert.equal(r.bitsC, 0b0011);

    // 4B 2D 31 (10 + 3 + 11) = 01001011 00|101|101 00110001
    assert.equal(r.largeBits1, 0b0100101100);
    assert.equal(r.spacer, 0b101);
    assert.equal(r.largeBits2, 0b10100110001);

    // FF FF
    assert.equal(r.normalS2, -1)

    // 50 41 43
    assert.equal(r.byte8910, 0x504143)
});
