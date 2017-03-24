var assert = require('assert');
var testHelper = require('testHelper');

testHelper('StrPadTerm', 'src/str_pad_term.bin', function(r) {
  assert.equal(r.strPad, 'str1');
  assert.equal(r.strTerm, 'str2foo');
  assert.equal(r.strTermAndPad, 'str+++3bar+++');
  assert.equal(r.strTermInclude, 'str4baz@');
});
