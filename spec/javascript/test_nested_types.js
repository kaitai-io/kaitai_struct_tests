var assert = require('assert');
var testHelper = require('testHelper');

testHelper('NestedTypes', 'src/fixed_struct.bin', function(r) {
  assert.equal(r.one.typedAtRoot.valueB, 80);
  assert.equal(r.one.typedHere.valueC, 65);
  assert.equal(r.two.valueB, 67);
});
