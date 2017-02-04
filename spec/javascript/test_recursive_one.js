var assert = require('assert');
var testHelper = require('testHelper');

testHelper('RecursiveOne', 'src/fixed_struct.bin', function(r) {
  assert.equal(r.one, 0x50);
  assert.equal(r.next.one, 0x41);
  assert.equal(r.next.next.one, 0x43);
  assert.equal(r.next.next.next.finisher, 0x2d4b);
});
