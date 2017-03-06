var assert = require('assert');
var testHelper = require('testHelper');

testHelper('ImportsCircularA', 'src/fixed_struct.bin', function(r) {
  assert.equal(r.code, 0x50);
  assert.equal(r.two.initial, 0x41);
  assert.equal(r.two.backRef.code, 0x43);
  assert.equal(r.two.backRef.two.initial, 0x4b);
  assert.strictEqual(r.two.backRef.two.backRef, null);
});
