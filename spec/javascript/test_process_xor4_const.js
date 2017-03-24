var assert = require('assert');
var testHelper = require('testHelper');

testHelper('ProcessXor4Const', 'src/process_xor_4.bin', function(r) {
    assert.deepEqual(r.key.toString(), new Uint8Array([0xec, 0xbb, 0xa3, 0x14]).toString());
    assert.equal(KaitaiStream.bytesToStr(r.buf, "UTF-8"), "foo bar");
});
