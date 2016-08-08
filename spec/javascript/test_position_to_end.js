var assert = require('assert');
var testHelper = require('testHelper');

testHelper('PositionToEnd', 'src/position_to_end.bin', function(r) {
    assert.equal(r.index.foo, 0x42);
    assert.equal(r.index.bar, 0x1234);
});
