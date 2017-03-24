var assert = require('assert');
var testHelper = require('testHelper');

testHelper('NavParentVsValueInst', 'src/term_strz.bin', function(r) {
  assert.equal(r.s1, 'foo');
});
