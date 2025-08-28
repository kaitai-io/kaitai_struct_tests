var assert = require('assert');
var testHelper = require('testHelper');
var KaitaiStream = require('kaitai-struct/KaitaiStream');

testHelper('DebugArrayUserEofException', 'src/nav_parent_codes.bin', function(r, DebugArrayUserEofException_) {
  assert.throws(function() {
    // --debug implies --no-auto-read
    r._read();
  }, KaitaiStream.EOFError);

  assert.strictEqual(r.oneCat.meow, 3);
  assert.strictEqual(r.oneCat.chirp, 73);
  assert.strictEqual(r.arrayOfCats.length, 3);
  assert.strictEqual(r.arrayOfCats[0].meow, 49);
  assert.strictEqual(r.arrayOfCats[0].chirp, 50);
  assert.strictEqual(r.arrayOfCats[1].meow, 51);
  assert.strictEqual(r.arrayOfCats[1].chirp, 66);
  assert.strictEqual(r.arrayOfCats[2].meow, 98);
});
