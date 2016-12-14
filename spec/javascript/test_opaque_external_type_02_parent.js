var assert = require('assert');
var testHelper = require('testHelper');

testHelper('OpaqueExternalType02Parent', 'src/term_strz.bin', function(r) {
  assert.equal(r.parent.child.s1, 'foo');
  assert.equal(r.parent.child.s2, 'bar');
  assert.equal(r.parent.child.s3.s3, '|baz@');
});
