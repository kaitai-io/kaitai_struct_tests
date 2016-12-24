var assert = require('assert');
var testHelper = require('testHelper');

testHelper('SwitchMultiBoolOps', 'src/switch_integers.bin', function(r) {
  assert.equal(r.opcodes.length, 4);

  assert.equal(r.opcodes[0].code, 1);
  assert.equal(r.opcodes[0].body, 7);

  assert.equal(r.opcodes[1].code, 2);
  assert.equal(r.opcodes[1].body, 0x4040);

  assert.equal(r.opcodes[2].code, 4);
  assert.equal(r.opcodes[2].body, 4919);

  assert.equal(r.opcodes[3].code, 8);
  assert.equal(r.opcodes[3].body, 4919);
});
