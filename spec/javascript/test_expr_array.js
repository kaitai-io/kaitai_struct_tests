var assert = require('assert');
var testHelper = require('testHelper');

testHelper('ExprArray', 'src/expr_array.bin', function(r) {
  assert.equal(r.aintSize, 4);
  assert.equal(r.aintFirst, 7657765);
  assert.equal(r.aintLast, 16272640);
  assert.equal(r.aintMin, 49185);
  assert.equal(r.aintMax, 1123362332);

  assert.equal(r.afloatSize, 3);
  assert.equal(r.afloatFirst, -2.6839530254859364e-121);
  assert.equal(r.afloatLast, -1.1103359815095273e-175);
  assert.equal(r.afloatMin, -8.754689149998834e+288);
  assert.equal(r.afloatMax, -1.1103359815095273e-175);

  assert.equal(r.astrSize, 3);
  assert.equal(r.astrFirst, 'foo');
  assert.equal(r.astrLast, 'baz');
  assert.equal(r.astrMin, 'bar');
  assert.equal(r.astrMax, 'foo');
});
