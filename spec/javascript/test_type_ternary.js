var assert = require('assert');
var testHelper = require('testHelper');

testHelper('TypeTernary', 'src/term_strz.bin', function(r) {
  assert.equal(r.dif.value, 0x65);
});
