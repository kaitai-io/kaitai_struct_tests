var assert = require('assert');
var testHelper = require('testHelper');

testHelper('IndexSizes', 'src/index_sizes.bin', function(r) {
  assert.equal(r.qty, 3);

  assert.equal(r.sizes[0], 1);
  assert.equal(r.sizes[1], 8);
  assert.equal(r.sizes[2], 4);

  assert.equal(r.bufs[0], "A");
  assert.equal(r.bufs[1], "BBBBBBBB");
  assert.equal(r.bufs[2], "CCCC");
});
