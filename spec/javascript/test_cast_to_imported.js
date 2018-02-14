var assert = require('assert');
var testHelper = require('testHelper');

testHelper('CastToImported', 'src/fixed_struct.bin', function(r) {
  assert.equal(r.one.one, 0x50);
  assert.equal(r.oneCasted.one, 0x50);
});
