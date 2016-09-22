var assert = require('assert');
var testHelper = require('testHelper');

testHelper('NestedSameName2', 'src/nested_same_name2.bin', function(r) {
  assert.equal(r.version, 0x42);
  assert.equal(r.mainData.mainSize, 2);
  assert.equal(r.mainData.foo.data1.toString(), [0x11, 0x11, 0x11, 0x11].toString());
  assert.equal(r.dummy.dummySize, 3);
  assert.equal(r.dummy.foo.data2.toString(), [0x22, 0x22, 0x22, 0x22, 0x22, 0x22].toString());
});
