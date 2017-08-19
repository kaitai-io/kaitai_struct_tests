var assert = require('assert');
var testHelper = require('testHelper');

testHelper('JsSignedRightShift', 'src/fixed_struct.bin', function(r) {
  assert.equal(r.shouldBe40000000, 0x40000000);
  assert.equal(r.shouldBeA00000, 0xa00000);
});
