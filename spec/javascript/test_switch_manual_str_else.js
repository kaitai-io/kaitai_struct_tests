var assert = require('assert');
var testHelper = require('testHelper');

testHelper('SwitchManualStrElse', 'src/switch_opcodes2.bin', function(r) {
  assert.equal(r.opcodes.length, 4);

  assert.equal(r.opcodes[0].code, 'S');
  assert.equal(r.opcodes[0].body.value, 'foo');

  assert.equal(r.opcodes[1].code, 'X');
  assert.equal(r.opcodes[1].body.filler, 0x42);

  assert.equal(r.opcodes[2].code, 'Y');
  assert.equal(r.opcodes[2].body.filler, 0xcafe);

  assert.equal(r.opcodes[3].code, 'I');
  assert.equal(r.opcodes[3].body.value, 7);
});
