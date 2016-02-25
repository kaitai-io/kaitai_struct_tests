var assert = require('assert');
var testHelper = require('testHelper')

testHelper('PositionAbs', 'src/position_abs.bin', function(r) {
    assert.equal(0x20, r.indexOffset)
    assert.equal("foo", r.index().entry)
});
