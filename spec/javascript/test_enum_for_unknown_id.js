var assert = require('assert');
var testHelper = require('testHelper');

testHelper('EnumForUnknownId', 'src/fixed_struct.bin', function(r) {
  assert.equal(r.one, 80);
});
