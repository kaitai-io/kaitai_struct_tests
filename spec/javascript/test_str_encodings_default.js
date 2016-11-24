var assert = require('assert');
var testHelper = require('testHelper');

testHelper('StrEncodingsDefault', 'src/str_encodings.bin', function(r) {
  assert.equal(r.str1, 'Some ASCII');
  assert.equal(r.rest.str2, 'こんにちは');
  assert.equal(r.rest.str3, 'こんにちは');
  assert.equal(r.rest.str4, '░▒▓');
});
