var assert = require('assert');
var testHelper = require('testHelper');

testHelper('StrLiterals2', 'src/fixed_struct.bin', function(r) {
  assert.equal(r.dollar1, '$foo');
  assert.equal(r.dollar2, '${foo}');
  assert.equal(r.hash, '#{foo}');
  assert.equal(r.atSign, '@foo');
});
