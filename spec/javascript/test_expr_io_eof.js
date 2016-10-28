var assert = require('assert');
var testHelper = require('testHelper');

testHelper('ExprIoEof', 'src/fixed_struct.bin', function(r) {
  assert.equal(r.substream1.one, 1262698832);
  assert.strictEqual(r.substream1.two, undefined);

  assert.equal(r.substream2.one, 4294914349);
  assert.equal(r.substream2.two, 1262698832);
});
