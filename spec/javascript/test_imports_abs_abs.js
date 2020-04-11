var assert = require('assert');
var testHelper = require('testHelper');

testHelper('ImportsAbsAbs', 'src/fixed_struct.bin', function(r, ImportsAbsAbs) {

  assert.strictEqual(r.one, 80);
  assert.strictEqual(r.two.one, 65);
  assert.strictEqual(r.two.two.one, 67);
});
