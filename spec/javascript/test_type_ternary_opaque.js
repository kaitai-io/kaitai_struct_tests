var assert = require('assert');
var testHelper = require('testHelper');

testHelper('TypeTernaryOpaque', 'src/term_strz.bin', function(r) {
  assert.strictEqual(r.dif.s1, 'foo');
  assert.strictEqual(r.dif.s2, 'bar');
  assert.strictEqual(r.dif.s3, '|baz@');
});
