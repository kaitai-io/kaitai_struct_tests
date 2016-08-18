var assert = require('assert');
var testHelper = require('testHelper');

testHelper('MultipleUse', 'src/position_abs.bin', function(r) {
  assert.equal(r.t1.firstUse.value, 0x20);
  assert.equal(r.t2.secondUse.value, 0x20);
});
