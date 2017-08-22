var assert = require('assert');
var testHelper = require('testHelper');

testHelper('CastToTop', 'src/fixed_struct.bin', function(r) {
  assert.equal(r.code, 0x50);
  assert.equal(r.header.code, 0x41);
  assert.equal(r.headerCasted.code, 0x41);
});
