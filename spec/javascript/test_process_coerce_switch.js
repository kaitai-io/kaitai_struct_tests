var assert = require('assert');
var testHelper = require('testHelper');

testHelper('ProcessCoerceSwitch', 'src/process_coerce_switch.bin', function(r) {
    assert.equal(r.bufType, 0);
    assert.equal(r.flag, 0);
    assert.equal(KaitaiStream.bytesToStr(r.buf.bar, "UTF-8"), "AAAA");
});
