var assert = require('assert');
var testHelper = require('testHelper');

testHelper('DefaultEndianExprInherited', 'src/endian_expr.bin', function(r) {
  assert.equal(r.docs[0].indicator.toString(), [0x49, 0x49].toString());
  assert.equal(r.docs[0].main.insides.someInt, 0x42);
  assert.equal(r.docs[0].main.insides.more.someInt1, 0x4200);
  assert.equal(r.docs[0].main.insides.more.someInt2, 0x42);
  assert.equal(r.docs[0].main.insides.more.someInst, 0x42);

  assert.equal(r.docs[1].indicator.toString(), [0x4d, 0x4d].toString());
  assert.equal(r.docs[1].main.insides.someInt, 0x42);
  assert.equal(r.docs[1].main.insides.more.someInt1, 0x42);
  assert.equal(r.docs[1].main.insides.more.someInt2, 0x4200);
  assert.equal(r.docs[1].main.insides.more.someInst, 0x42000000);

  assert.equal(r.docs[2].indicator.toString(), [0x58, 0x58].toString());
  assert.equal(r.docs[2].main.insides.someInt, 0x42);
  assert.equal(r.docs[2].main.insides.more.someInt1, 0x42);
  assert.equal(r.docs[2].main.insides.more.someInt2, 0x4200);
  assert.equal(r.docs[2].main.insides.more.someInst, 0x42000000);
});
