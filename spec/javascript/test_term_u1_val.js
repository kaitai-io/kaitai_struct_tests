// Autogenerated from KST: please remove this line if doing any edits by hand!

var assert = require('assert');
var testHelper = require('testHelper');
var hexString = require('hexString');

testHelper('TermU1Val', 'src/str_encodings.bin', function(r, TermU1Val) {

  assert.strictEqual(hexString(r.foo), hexString([10, 0, 83, 111, 109, 101, 32, 65, 83, 67, 73, 73, 15, 0]));
  assert.strictEqual(r.bar, "\u3053\u3093\u306b");
});
