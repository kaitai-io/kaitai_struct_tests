var assert = require('assert');
var testHelper = require('testHelper');

testHelper('SwitchManualIntElse', 'src/switch_opcodes2.bin', function(r) {
  assert.equal(r.opcodes.length, 4);

  assert.equal(r.opcodes[0].code, 83);
  assert.equal(r.opcodes[0].body.value, 'foo');

  assert.equal(r.opcodes[1].code, 88);
  assert.equal(r.opcodes[1].body.filler, 0x42);

  assert.equal(r.opcodes[2].code, 89);
  assert.equal(r.opcodes[2].body.filler, 0xcafe);

  assert.equal(r.opcodes[3].code, 73);
  assert.equal(r.opcodes[3].body.value, 7);
});
