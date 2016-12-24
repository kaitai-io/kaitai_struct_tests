var assert = require('assert');
var testHelper = require('testHelper');

testHelper('EnumOfValueInst', 'src/enum_0.bin', function(r, EnumOfValueInst) {
  assert.equal(r.pet1, EnumOfValueInst.Animal.CAT);
  assert.equal(r.pet2, EnumOfValueInst.Animal.CHICKEN);
  assert.equal(r.pet3, EnumOfValueInst.Animal.DOG);
  assert.equal(r.pet4, EnumOfValueInst.Animal.DOG);
});
