var assert = require('assert');
var testHelper = require('testHelper');

testHelper('OptionalId', 'src/fixed_struct.bin', function(r, OptionalId_) {
  assert.strictEqual(r._unnamed0, 80);
  assert.strictEqual(r._unnamed1, 65);
  assert.deepStrictEqual(r._unnamed2, new Uint8Array([67, 75, 45, 49, 255]));
});
