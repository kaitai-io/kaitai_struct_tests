var assert = require('assert');
var testHelper = require('testHelper');

testHelper('EnumNegative', 'src/enum_negative.bin', function(r, EnumNegative) {
  assert.equal(r.f1, EnumNegative.Constants.NEGATIVE_ONE);
  assert.equal(r.f2, EnumNegative.Constants.POSITIVE_ONE);
});
