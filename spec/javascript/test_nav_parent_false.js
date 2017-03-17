var assert = require('assert');
var testHelper = require('testHelper');

testHelper('NavParentFalse', 'src/nav_parent_codes.bin', function(r) {
  assert.equal(r.childSize, 3);
  assert.equal(r.elementA.foo.code, 73);
  assert.equal(r.elementA.foo.more.toString(), [49, 50, 51].toString());
  assert.equal(r.elementA.bar.foo.code, 66);
  assert.equal(r.elementB.foo.code, 98);
});
