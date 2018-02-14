var assert = require('assert');
var testHelper = require('testHelper');

testHelper('DefaultEndianExprIsBe', 'src/endian_expr.bin', function(r) {
  // LE
  assert.equal(r.docs[0].indicator.toString(), [0x49, 0x49].toString());
  assert.equal(r.docs[0].main.someInt, 0x42);
  assert.equal(r.docs[0].main.someIntBe, 0x42);
  assert.equal(r.docs[0].main.someIntLe, 0x42);

  assert.equal(r.docs[0].main.instInt, 0x42);
  assert.equal(r.docs[0].main.instSub.foo, 0x42);

  // BE
  assert.equal(r.docs[1].indicator.toString(), [0x4d, 0x4d].toString());
  assert.equal(r.docs[1].main.someInt, 0x42);
  assert.equal(r.docs[1].main.someIntBe, 0x42);
  assert.equal(r.docs[1].main.someIntLe, 0x42);

  assert.equal(r.docs[1].main.instInt, 0x42000000);
  assert.equal(r.docs[1].main.instSub.foo, 0x42000000);

  // Weird => LE
  assert.equal(r.docs[2].indicator.toString(), [0x58, 0x58].toString());
  assert.equal(r.docs[2].main.someInt, 0x42000000);
  assert.equal(r.docs[2].main.someIntBe, 0x42);
  assert.equal(r.docs[2].main.someIntLe, 0x42);

  assert.equal(r.docs[2].main.instInt, 0x42);
  assert.equal(r.docs[2].main.instSub.foo, 0x42);
});
