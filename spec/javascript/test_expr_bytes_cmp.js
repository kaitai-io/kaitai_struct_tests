var assert = require('assert');
var testHelper = require('testHelper');

testHelper('ExprBytesCmp', 'src/fixed_struct.bin', function(r) {
  assert.equal(KaitaiStream.bytesToStr(r.one, "UTF-8"), "P");
  assert.equal(KaitaiStream.bytesToStr(r.two, "UTF-8"), "ACK");

  assert.equal(r.isEq, true);
  assert.equal(r.isNe, false);
  assert.equal(r.isLt, true);
  assert.equal(r.isGt, false);
  assert.equal(r.isLe, true);
  assert.equal(r.isGe, false);
  assert.equal(r.isLt2, false);
  assert.equal(r.isGt2, true);
});
