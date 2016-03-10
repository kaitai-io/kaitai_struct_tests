var assert = require('assert');
var testHelper = require('testHelper');

testHelper('DefaultBigEndian', 'src/enum_0.bin', function(r) {
  assert.equal(r.one, 0x7000000);
});
