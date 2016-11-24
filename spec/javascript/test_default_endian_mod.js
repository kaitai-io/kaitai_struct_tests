var assert = require('assert');
var testHelper = require('testHelper');

testHelper('DefaultEndianMod', 'src/fixed_struct.bin', function(r) {
  assert.equal(r.main.one, 0x4b434150);
  assert.equal(r.main.nest.two, -52947);
  assert.equal(r.main.nestBe.two, 0x5041434b);
});
