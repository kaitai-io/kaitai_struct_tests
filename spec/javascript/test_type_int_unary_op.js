var assert = require('assert');
var testHelper = require('testHelper');

testHelper('TypeIntUnaryOp', 'src/fixed_struct.bin', function(r) {
  assert.equal(r.valueS2, 0x4150);
  assert.equal(r.valueS8, 0x4150ffff312d4b43);
  assert.equal(r.unaryS2, -0x4150);
  assert.equal(r.unaryS8, -0x4150ffff312d4b43);
});
