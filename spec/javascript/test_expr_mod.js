var assert = require('assert');
var testHelper = require('testHelper');

testHelper('ExprMod', 'src/fixed_struct.bin', function(r) {
  assert.equal(r.intU, 1262698832);
  assert.equal(r.intS, -52947);

  assert.equal(r.modPosConst, 9);
  assert.equal(r.modNegConst, 4);
  assert.equal(r.modPosSeq, 5);
  assert.equal(r.modNegSeq, 2);
});
