var assert = require('assert');
var testHelper = require('testHelper');

testHelper('OpaqueWithParam', 'src/term_strz.bin', function(r) {
  assert.strictEqual(r.one.buf, 'foo|b');
  assert.strictEqual(r.one.trailer, 0x61);
});
