var assert = require('assert');
var testHelper = require('testHelper');

testHelper('RepeatUntilS4', 'src/repeat_until_s4.bin', function(r) {
  assert.equal(r.entries, [0x42, 0x1337, -251658241, -1]);
  assert.equal(r.afterall, "foobar");
});
