// runs in debug mode, so the _read() needs to be called manually

var assert = require('assert');
var testHelper = require('testHelper');

testHelper('DebugSwitchUser', 'src/nav_parent_switch.bin', function(r, DebugSwitchUser) {
  r._read();

  assert.strictEqual(r.code, 1);
  assert.strictEqual(r.data.val, -190);
});
