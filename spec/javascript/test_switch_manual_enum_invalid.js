// Autogenerated from KST: please remove this line if doing any edits by hand!

var assert = require('assert');
var testHelper = require('testHelper');

testHelper('SwitchManualEnumInvalid', 'src/enum_negative.bin', function(r, SwitchManualEnumInvalid_) {
  assert.strictEqual(r.opcodes.length, 2);
  assert.strictEqual(r.opcodes[0].code, 255);
  assert.strictEqual(r.opcodes[0].body, undefined);
  assert.strictEqual(r.opcodes[1].code, 1);
  assert.strictEqual(r.opcodes[1].body, undefined);
});
