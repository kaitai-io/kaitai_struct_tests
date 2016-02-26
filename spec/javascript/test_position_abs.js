var assert = require('assert');
var testHelper = require('testHelper');

testHelper('PositionAbs', 'src/position_abs.bin', function(r) {
    assert.equal(r.indexOffset, 0x20);
    assert.equal(r.index().entry, "foo");
});
