// Autogenerated from KST: please remove this line if doing any edits by hand!

var assert = require('assert');
var testHelper = require('testHelper');

testHelper('DefaultEndianExprIsBe', 'src/endian_expr.bin', function(r, DefaultEndianExprIsBe_) {
  assert.deepStrictEqual(r.docs[0].indicator, new Uint8Array([73, 73]));
  assert.strictEqual(r.docs[0].main.someInt, 66);
  assert.strictEqual(r.docs[0].main.someIntBe, 66);
  assert.strictEqual(r.docs[0].main.someIntLe, 66);
  assert.strictEqual(r.docs[0].main.instInt, 66);
  assert.strictEqual(r.docs[0].main.instSub.foo, 66);
  assert.deepStrictEqual(r.docs[1].indicator, new Uint8Array([77, 77]));
  assert.strictEqual(r.docs[1].main.someInt, 66);
  assert.strictEqual(r.docs[1].main.someIntBe, 66);
  assert.strictEqual(r.docs[1].main.someIntLe, 66);
  assert.strictEqual(r.docs[1].main.instInt, 1107296256);
  assert.strictEqual(r.docs[1].main.instSub.foo, 1107296256);
  assert.deepStrictEqual(r.docs[2].indicator, new Uint8Array([88, 88]));
  assert.strictEqual(r.docs[2].main.someInt, 1107296256);
  assert.strictEqual(r.docs[2].main.someIntBe, 66);
  assert.strictEqual(r.docs[2].main.someIntLe, 66);
  assert.strictEqual(r.docs[2].main.instInt, 66);
  assert.strictEqual(r.docs[2].main.instSub.foo, 66);
});
