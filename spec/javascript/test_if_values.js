var assert = require('assert');
var testHelper = require('testHelper');

testHelper('IfValues', 'src/fixed_struct.bin', function(r) {
  assert.equal(r.codes[0].opcode, 80);
  assert.equal(r.codes[0].halfOpcode, 40);
  assert.equal(r.codes[1].opcode, 65);
  assert.strictEqual(r.codes[1].halfOpcode, undefined);
  assert.equal(r.codes[2].opcode, 67);
  assert.strictEqual(r.codes[2].halfOpcode, undefined);
});
