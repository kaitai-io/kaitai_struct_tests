var assert = require('assert');
var testHelper = require('testHelper');

testHelper('RepeatNStrz', 'src/repeat_n_strz.bin', function(r) {
    assert.equal(r.qty, 2);
    assert.deepEqual(r.lines, ["foo", "bar"]);
});
