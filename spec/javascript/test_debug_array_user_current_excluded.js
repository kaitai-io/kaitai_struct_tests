var assert = require('assert');
var testHelper = require('testHelper');

testHelper('DebugArrayUserCurrentExcluded', 'src/term_strz.bin', function(r, DebugArrayUserCurrentExcluded_) {
  // --debug implies --no-auto-read
  r._read();

  assert.deepStrictEqual(r.arrayOfCats[0].meow, new Uint8Array([102, 111, 111]));
  assert.deepStrictEqual(r.arrayOfCats[1].meow, new Uint8Array([124, 98]));
  assert.deepStrictEqual(r.arrayOfCats[2].meow, new Uint8Array([97]));
});
