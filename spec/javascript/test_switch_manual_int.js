var assert = require('assert');
var testHelper = require('testHelper');

testHelper('SwitchManualInt', 'src/switch_opcodes.bin', function(r) {
  assert.equal(r.opcodes.length, 4);

  assert.equal(r.opcodes[0].code, 83);
  assert.equal(r.opcodes[0].body.value, 'foobar');

  assert.equal(r.opcodes[1].code, 73);
  assert.equal(r.opcodes[1].body.value, 0x42);

  assert.equal(r.opcodes[2].code, 73);
  assert.equal(r.opcodes[2].body.value, 0x37);

  assert.equal(r.opcodes[3].code, 83);
  assert.equal(r.opcodes[3].body.value, '');
});
