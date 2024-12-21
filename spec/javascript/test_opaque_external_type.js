var assert = require('assert');
var testHelper = require('testHelper');

testHelper('OpaqueExternalType', 'src/term_strz.bin', function(r) {
  assert.strictEqual(r.one.s1, 'foo');
  assert.strictEqual(r.one.s2, 'bar');
  assert.strictEqual(r.one.s3, '|baz@');
});
