var assert = require('assert');
var testHelper = require('testHelper');

testHelper('ProcessCustom', 'src/process_rotate.bin', function(r) {
    assert.equal(r.buf1.toString(), new Uint8Array([0x10, 0xb3, 0x94, 0x94, 0xf4]).toString());
    assert.equal(r.buf2.toString(), new Uint8Array([0x5f, 0xba, 0x7b, 0x93, 0x63, 0x23, 0x5f]).toString());
    assert.equal(r.buf3.toString(), new Uint8Array([0x29, 0x33, 0xb1, 0x38, 0xb1]).toString());
});
