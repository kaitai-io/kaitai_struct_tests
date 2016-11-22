var assert = require('assert');
var testHelper = require('testHelper');

testHelper('OpaqueExternalType', 'src/term_strz.bin', function(r) {
  assert.equal(r.one.s1, 'foo');
  assert.equal(r.one.s2, 'bar');
  assert.equal(r.one.s3, '|baz@');
});
