var assert = require('assert');
var testHelper = require('testHelper');

testHelper('ImportsAbs', 'src/fixed_struct.bin', function(r) {
  assert.equal(r.len.value, 80);
  assert.equal(r.body.length, 80);
});
