var assert = require('assert');
var testHelper = require('testHelper');

testHelper('Expr3', 'src/fixed_struct.bin', function(r) {
  assert.equal(r.one, 80);
  assert.equal(r.two, 'ACK');

  assert.equal(r.three, '@ACK');
  assert.equal(r.four, '_ACK_');
  assert.equal(r.isStrEq, true);
  assert.equal(r.isStrNe, false);
  assert.equal(r.isStrLt, true);
  assert.equal(r.isStrGt, false);
  assert.equal(r.isStrLe, true);
  assert.equal(r.isStrGe, false);
  assert.equal(r.isStrLt2, true);
  assert.equal(r.testNot, true);
});
