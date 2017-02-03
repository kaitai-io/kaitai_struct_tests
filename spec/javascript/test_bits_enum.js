var assert = require('assert');
var testHelper = require('testHelper');

testHelper('BitsEnum', 'src/fixed_struct.bin', function(r, BitsEnum) {
  // 50 41 (4 + 8 + 1) = 0101|0000 0100|0|001
  assert.equal(r.one, BitsEnum.Animal.PLATYPUS);
  assert.equal(r.two, BitsEnum.Animal.HORSE);
  assert.equal(r.three, BitsEnum.Animal.CAT);
});
