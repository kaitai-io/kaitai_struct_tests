var assert = require('assert');
var testHelper = require('testHelper');

testHelper('ExprIoPos', 'src/expr_io_pos.bin', function(r) {
  assert.equal(r.substream1.myStr, 'CURIOSITY');
  assert.equal(r.substream1.body.toString(), new Uint8Array([0x11, 0x22, 0x33, 0x44]).toString());
  assert.equal(r.substream1.number, 0x42);

  assert.equal(r.substream2.myStr, 'KILLED');
  assert.equal(r.substream2.body.toString(), new Uint8Array([0x61, 0x20, 0x63, 0x61, 0x74]).toString());
  assert.equal(r.substream2.number, 0x67);
});
