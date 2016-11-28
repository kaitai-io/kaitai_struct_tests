var assert = require('assert');
var testHelper = require('testHelper');

testHelper('ExprEnum', 'src/term_strz.bin', function(r, ExprEnum) {
  assert.equal(r.constDog, ExprEnum.Animal.DOG);
  assert.equal(r.derivedBoom, ExprEnum.Animal.BOOM);
  assert.equal(r.derivedDog, ExprEnum.Animal.DOG);
});
