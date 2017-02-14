var assert = require('assert');
var testHelper = require('testHelper');

testHelper('ProcessRotate', 'src/process_rotate.bin', function(r) {
    assert.equal(KaitaiStream.bytesToStr(r.buf1, "UTF-8"), "Hello");
    assert.equal(KaitaiStream.bytesToStr(r.buf2, "UTF-8"), "World");
    assert.equal(KaitaiStream.bytesToStr(r.buf3, "UTF-8"), "There");
});
