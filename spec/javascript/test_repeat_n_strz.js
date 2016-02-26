var assert = require('assert');
var testHelper = require('testHelper');

testHelper('RepeatNStrz', 'src/repeat_n_strz.bin', function(r) {
    assert.equal(2, r.qty);
    assert.deepEqual(["foo", "bar"], r.lines);
});
