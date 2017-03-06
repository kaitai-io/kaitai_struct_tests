var assert = require('assert');
var testHelper = require('testHelper');

testHelper('Imports0', 'src/fixed_struct.bin', function(r) {
  assert.equal(r.two, 0x50);
  assert.equal(r.hw.one, 0x41);
  assert.equal(r.hwOne, 0x41);
});
