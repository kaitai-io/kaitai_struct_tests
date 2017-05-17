var assert = require('assert');
var testHelper = require('testHelper');

testHelper('SwitchBytearray', 'src/switch_opcodes.bin', function(r) {
  assert.equal(r.opcodes.length, 4);

  assert.equal(r.opcodes[0].code.toString(), [83].toString());
  assert.equal(r.opcodes[0].body.value, 'foobar');

  assert.equal(r.opcodes[1].code.toString(), [73].toString());
  assert.equal(r.opcodes[1].body.value, 0x42);

  assert.equal(r.opcodes[2].code.toString(), [73].toString());
  assert.equal(r.opcodes[2].body.value, 0x37);

  assert.equal(r.opcodes[3].code.toString(), [83].toString());
  assert.equal(r.opcodes[3].body.value, '');
});
