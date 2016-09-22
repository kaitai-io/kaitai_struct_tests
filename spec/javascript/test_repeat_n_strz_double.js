var assert = require('assert');
var testHelper = require('testHelper');

testHelper('RepeatNStrzDouble', 'src/repeat_n_strz.bin', function(r) {
  assert.equal(r.qty, 2);
  assert.deepEqual(r.lines1, ['foo']);
  assert.deepEqual(r.lines2, ['bar']);
});
