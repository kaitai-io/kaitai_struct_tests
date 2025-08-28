var assert = require('assert');
var testHelper = require('testHelper');

testHelper('SwitchCast', 'src/switch_opcodes.bin', function(r) {
  assert.strictEqual(r.firstObj.value, 'foobar');
  assert.strictEqual(r.secondVal, 0x42);
  // unable to test "err_cast" here
});
