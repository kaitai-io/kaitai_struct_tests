var assert = require('assert');
var testHelper = require('testHelper');

testHelper('DebugArrayUser', 'src/fixed_struct.bin', function(r) {
    r._read();

    assert.equal(r.oneCat.meow, 80);
    assert.equal(r.arrayOfCats[0].meow, 65);
    assert.equal(r.arrayOfCats[1].meow, 67);
    assert.equal(r.arrayOfCats[2].meow, 75);
});
