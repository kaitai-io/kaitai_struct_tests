var assert = require('assert');
var testHelper = require('testHelper');

testHelper('NavParentFalse2', 'src/fixed_struct.bin', function(r) {
  assert.equal(r.parentless.foo, 80);
});
