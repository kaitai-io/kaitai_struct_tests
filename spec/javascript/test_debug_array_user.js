var assert = require('assert');
var testHelper = require('testHelper');

testHelper('DebugArrayUser', 'src/fixed_struct.bin', function(r, DebugArrayUser_) {
  // --debug implies --no-auto-read
  r._read();

  assert.strictEqual(r.oneCat.meow, 80);
  assert.strictEqual(r.arrayOfCats.length, 3);
  assert.strictEqual(r.arrayOfCats[0].meow, 65);
  assert.strictEqual(r.arrayOfCats[1].meow, 67);
  assert.strictEqual(r.arrayOfCats[2].meow, 75);
});
