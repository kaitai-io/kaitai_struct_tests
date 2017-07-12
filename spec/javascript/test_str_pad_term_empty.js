var assert = require('assert');
var testHelper = require('testHelper');

testHelper('StrPadTermEmpty', 'src/str_pad_term_empty.bin', function(r) {
  assert.equal(r.strPad, '');
  assert.equal(r.strTerm, '');
  assert.equal(r.strTermAndPad, '');
  assert.equal(r.strTermInclude, '@');
});
