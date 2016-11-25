var assert = require('assert');
var testHelper = require('testHelper');

testHelper('OptionalId', 'src/fixed_struct.bin', function(r) {
  assert.equal(r._unnamed0, 0x50);
  assert.equal(r._unnamed1, 0x41);
  assert.equal(r._unnamed2.toString(), [0x43, 0x4b, 0x2d, 0x31, 0xff].toString());
});
