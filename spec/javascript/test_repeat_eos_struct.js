var assert = require('assert');
var testHelper = require('testHelper');

testHelper('RepeatEosStruct', 'src/repeat_eos_struct.bin', function(r) {
    assert.equal(r.chunks.length, 2);
    assert.equal(r.chunks[0].offset, 0);
    assert.equal(r.chunks[0].len, 0x42);
    assert.equal(r.chunks[1].offset, 0x42);
    assert.equal(r.chunks[1].len, 0x815);
});
