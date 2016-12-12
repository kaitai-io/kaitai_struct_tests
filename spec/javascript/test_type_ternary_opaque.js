var assert = require('assert');
var testHelper = require('testHelper');

testHelper('TypeTernaryOpaque', 'src/term_strz.bin', function(r) {
  assert.equal(r.dif.s1, 'foo');
  assert.equal(r.dif.s2, 'bar');
  assert.equal(r.dif.s3, '|baz@');
});
