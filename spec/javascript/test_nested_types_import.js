var assert = require('assert');
var testHelper = require('testHelper');

testHelper('NestedTypesImport', 'src/fixed_struct.bin', function(r, NestedTypesImport_) {
  assert.strictEqual(r.aCc.valueCc, 80);
  assert.strictEqual(r.aCD.valueD, 65);
  assert.strictEqual(r.b.valueB, 67);
  assert.strictEqual(r.b.aCc.valueCc, 75);
  assert.strictEqual(r.b.aCD.valueD, 45);
  assert.strictEqual(r.aCc._parent, null);
  assert.strictEqual(r.aCc._root, null);
  assert.strictEqual(r.aCD._parent, null);
  assert.strictEqual(r.aCD._root, null);
  assert.strictEqual(r.b._parent, null);
  assert.strictEqual(r.b._root, null);
});
