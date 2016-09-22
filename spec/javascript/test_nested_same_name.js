var assert = require('assert');
var testHelper = require('testHelper');

testHelper('NestedSameName', 'src/repeat_n_struct.bin', function(r) {
  assert.equal(r.mainData.mainSize, 2);
  assert.equal(r.mainData.foo.data.toString(), [0x10, 0, 0, 0].toString());
});
