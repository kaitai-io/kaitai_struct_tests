var assert = require('assert');
var testHelper = require('testHelper');

testHelper('IfInstances', 'src/fixed_struct.bin', function(r) {
  assert.strictEqual(r.neverHappens, undefined);
});
