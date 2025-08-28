var assert = require('assert');
var testHelper = require('testHelper');

testHelper('TypeTernary2ndFalsy', 'src/switch_integers.bin', function(r, TypeTernary2ndFalsy_) {
  assert.strictEqual(r.vFalse, false);
  assert.strictEqual(r.vIntZero, 0);
  assert.strictEqual(r.vIntNegZero, 0);
  assert.strictEqual(r.vFloatZero, 0.0);
  assert.strictEqual(r.vFloatNegZero, -0.0);
  assert.strictEqual(r.vStrWZero, "0");
  assert.strictEqual(r.vStrWZero.length, 1);
  assert.strictEqual(r.ut.m, 7);
  assert.strictEqual(r.vNullUt, undefined);
  assert.strictEqual(r.vStrEmpty, "");
  assert.strictEqual(r.vStrEmpty.length, 0);
  assert.strictEqual(r.intArray.length, 2);
  assert.strictEqual(r.vIntArrayEmpty.length, 0);
});
