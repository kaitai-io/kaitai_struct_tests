var assert = require('assert');
var testHelperStream = require('testHelperStream');

testHelperStream('ParamsDef', 'src/term_strz.bin', function(io, ParamsDef) {
  var r = new ParamsDef(io, null, null, 5, true);

  assert.strictEqual(r.buf, 'foo|b');
  assert.strictEqual(r.trailer, 0x61);
});
