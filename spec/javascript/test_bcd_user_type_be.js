var assert = require('assert');
var testHelper = require('testHelper');

testHelper('BcdUserTypeBe', 'src/bcd_user_type_be.bin', function(r) {
  assert.equal(r.ltr.asInt, 12345678);
  assert.equal(r.ltr.asStr, "12345678");
  assert.equal(r.rtl.asInt, 87654321);
  assert.equal(r.rtl.asStr, "87654321");
  assert.equal(r.leadingZeroLtr.asInt, 123456);
  assert.equal(r.leadingZeroLtr.asStr, "00123456");
});
