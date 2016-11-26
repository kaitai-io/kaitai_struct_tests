var assert = require('assert');
var testHelper = require('testHelper');

testHelper('Enum1', 'src/enum_0.bin', function(r, Enum1) {
  assert.equal(r.main.submain.pet1, Enum1.MainObj.Animal.CAT);
  assert.equal(r.main.submain.pet2, Enum1.MainObj.Animal.CHICKEN);
});
