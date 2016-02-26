var assert = require('assert');
var testHelper = require('testHelper');

testHelper('RepeatNStruct', 'src/repeat_n_struct.bin', function(r) {
    assert.equal(r.chunks.length, 2);
    assert.equal(r.chunks[0].offset, 0x10)
    assert.equal(r.chunks[0].len, 0x2078)
    assert.equal(r.chunks[1].offset, 0x2088)
    assert.equal(r.chunks[1].len, 0xf)
});
