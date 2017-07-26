var assert = require('assert');
var testHelper = require('testHelper');

testHelper('ParamsCallShort', 'src/term_strz.bin', function(r) {
  assert.equal(r.buf1.body, "foo|b");
  assert.equal(r.buf2.body, "ar|ba");
  assert.equal(r.buf2.trailer, 0x7a);
});
