var assert = require('assert');
var testHelper = require('testHelper');

testHelper('EnumToI', 'src/enum_0.bin', function(r, EnumToI) {
  assert.equal(r.pet1, EnumToI.Animal.CAT);
  assert.equal(r.pet2, EnumToI.Animal.CHICKEN);

  assert.equal(r.pet1I, 7);
  assert.equal(r.oneLtTwo, true);
});
