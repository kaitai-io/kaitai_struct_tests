var assert = require('assert');
var testHelper = require('testHelper');

testHelper('UserType', 'src/repeat_until_s4.bin', function(r) {
  assert.equal(r.one.width, 0x42);
  assert.equal(r.one.height, 0x1337);
});
