var assert = require('assert');
var testHelper = require('testHelper');

testHelper('NonStandard', 'src/fixed_struct.bin', function(r) {
  assert.equal(r.foo, 0x50);
});
