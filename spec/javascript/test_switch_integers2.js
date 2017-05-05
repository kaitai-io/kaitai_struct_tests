var assert = require('assert');
var testHelper = require('testHelper');

testHelper('SwitchIntegers2', 'src/switch_integers.bin', function(r) {
  assert.equal(r.code, 1);
  assert.equal(r.len, 7);
  assert.equal(r.ham.toString(), new Uint8Array([2, 64, 64, 4, 55, 19, 0]).toString());
  assert.equal(r.padding, 0);
  assert.equal(r.lenModStr, "13");
});
