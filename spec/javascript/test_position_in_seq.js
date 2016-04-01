var assert = require('assert');
var testHelper = require('testHelper');

testHelper('PositionInSeq', 'src/position_in_seq.bin', function(r) {
    assert.deepEqual(r.numbers, [1, 2, 3]);
});
