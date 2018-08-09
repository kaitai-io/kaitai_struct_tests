// Autogenerated from KST: please remove this line if doing any edits by hand!

var assert = require('assert');
var testHelper = require('testHelper');

testHelper('ExprArray', 'src/expr_array.bin', function(r) {
  assert.strictEqual(r.aintSize, 4);
  assert.strictEqual(r.aintFirst, 7657765);
  assert.strictEqual(r.aintLast, 16272640);
  assert.strictEqual(r.aintMin, 49185);
  assert.strictEqual(r.aintMax, 1123362332);
  assert.strictEqual(r.afloatSize, 3);
  assert.strictEqual(r.afloatFirst, -2.6839530254859364E-121);
  assert.strictEqual(r.afloatLast, -1.1103359815095273E-175);
  assert.strictEqual(r.afloatMin, -8.754689149998834E+288);
  assert.strictEqual(r.afloatMax, -1.1103359815095273E-175);
  assert.strictEqual(r.astrSize, 3);
  assert.strictEqual(r.astrFirst, "foo");
  assert.strictEqual(r.astrLast, "baz");
  assert.strictEqual(r.astrMin, "bar");
  assert.strictEqual(r.astrMax, "foo");
});
