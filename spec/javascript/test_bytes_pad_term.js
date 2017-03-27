var assert = require('assert');
var testHelper = require('testHelper');

testHelper('BytesPadTerm', 'src/str_pad_term.bin', function(r) {
  assert.equal(KaitaiStream.bytesToStr(r.strPad, "UTF-8"), 'str1');
  assert.equal(KaitaiStream.bytesToStr(r.strTerm, "UTF-8"), 'str2foo');
  assert.equal(KaitaiStream.bytesToStr(r.strTermAndPad, "UTF-8"), 'str+++3bar+++');
  assert.equal(KaitaiStream.bytesToStr(r.strTermInclude, "UTF-8"), 'str4baz@');
});
