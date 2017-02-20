var assert = require('assert');
var testHelper = require('testHelper');

testHelper('TermBytes', 'src/term_strz.bin', function(r) {
  assert.equal(KaitaiStream.bytesToStr(r.s1, "UTF-8"), 'foo');
  assert.equal(KaitaiStream.bytesToStr(r.s2, "UTF-8"), 'bar');
  assert.equal(KaitaiStream.bytesToStr(r.s3, "UTF-8"), '|baz@');
});
