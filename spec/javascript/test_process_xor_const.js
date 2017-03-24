var assert = require('assert');
var testHelper = require('testHelper');

testHelper('ProcessXorConst', 'src/process_xor_1.bin', function(r) {
    assert.equal(r.key, 0xff);
    assert.equal(KaitaiStream.bytesToStr(r.buf, "UTF-8"), "foo bar");
});
