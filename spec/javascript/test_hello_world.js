var assert = require('assert');
var testHelper = require('testHelper')

testHelper('HelloWorld', 'src/fixed_struct.bin', function(r) {
  assert.equal(0x50, r.one);
});
