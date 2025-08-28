var assert = require('assert');
var testHelper = require('testHelper');

testHelper('OpaqueExternalType02Parent', 'src/term_strz.bin', function(r, OpaqueExternalType02Parent_) {
  assert.strictEqual(r.parent.child.s1, "foo");
  assert.strictEqual(r.parent.child.s2, "bar");
  assert.strictEqual(r.parent.child.s3.s3, "|baz@");
});
