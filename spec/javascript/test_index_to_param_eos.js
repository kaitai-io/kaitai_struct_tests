var assert = require('assert');
var testHelper = require('testHelper');

testHelper('IndexToParamEos', 'src/index_sizes.bin', function(r) {
  assert.equal(r.qty, 3);

  assert.equal(r.sizes[0], 1);
  assert.equal(r.sizes[1], 8);
  assert.equal(r.sizes[2], 4);

  assert.equal(r.blocks[0].buf, "A");
  assert.equal(r.blocks[1].buf, "BBBBBBBB");
  assert.equal(r.blocks[2].buf, "CCCC");
});
