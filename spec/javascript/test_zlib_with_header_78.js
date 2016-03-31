var assert = require('assert');
var testHelper = require('testHelper');

testHelper('ZlibWithHeader78', 'src/zlib_with_header_78.bin', function(r) {
  assert.equal(r.data.toString("UTF-8"), "a quick brown fox jumps over");
});
