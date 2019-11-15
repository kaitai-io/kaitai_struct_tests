var assert = require('assert');
var testHelper = require('testHelper');

testHelper('OpaqueWithParam', 'src/term_strz.bin', function(r) {
  assert.equal(r.one.buf, 'foo|b');
  assert.equal(r.one.trailer, 0x61);
});
