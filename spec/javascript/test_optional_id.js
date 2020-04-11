var assert = require('assert');
var testHelper = require('testHelper');
var hexString = require('hexString');

testHelper('OptionalId', 'src/fixed_struct.bin', function(r, OptionalId) {

  assert.strictEqual(r._unnamed0, 80);
  assert.strictEqual(r._unnamed1, 65);
  assert.strictEqual(hexString(r._unnamed2), hexString([67, 75, 45, 49, 255]));
});
