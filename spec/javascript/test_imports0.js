// Autogenerated from KST: please remove this line if doing any edits by hand!

var assert = require('assert');
var testHelper = require('testHelper');

testHelper('Imports0', 'src/fixed_struct.bin', function(r, Imports0_) {
  assert.strictEqual(r.two, 80);
  assert.strictEqual(r.hw.one, 65);
  assert.strictEqual(r.hwOne, 65);
});
