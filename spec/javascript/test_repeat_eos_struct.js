var assert = require('assert');
var testHelper = require('testHelper');

testHelper('RepeatEosStruct', 'src/repeat_eos_struct.bin', function(r) {
    assert.equal(2, r.chunks.length);
    assert.equal(0, r.chunks[0].offset);
    assert.equal(0x42, r.chunks[0].len);
    assert.equal(0x42, r.chunks[1].offset);
    assert.equal(0x815, r.chunks[1].len);
});
