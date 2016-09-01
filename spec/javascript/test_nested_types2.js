var assert = require('assert');
var testHelper = require('testHelper');

testHelper('NestedTypes2', 'src/fixed_struct.bin', function(r) {
  assert.equal(r.one.typedAtRoot.valueB, 80);

  assert.equal(r.one.typedHere1.valueC, 65);

  assert.equal(r.one.typedHere1.typedHere.valueD, 67);
  assert.equal(r.one.typedHere1.typedParent.valueCc, 75);
  assert.equal(r.one.typedHere1.typedRoot.valueB, 45);

  assert.equal(r.one.typedHere2.valueCc, 49);

  assert.equal(r.two.valueB, -1);
});
