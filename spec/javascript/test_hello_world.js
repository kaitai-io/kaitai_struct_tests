var assert = require('assert');
var testHelper = require('testHelper');

testHelper('HelloWorld', 'src/fixed_struct.bin', function(r) {
  assert.equal(r.one, 0x50);
});
