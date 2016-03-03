var assert = require('assert');
var testHelper = require('testHelper');

testHelper('InstanceStdArray', 'src/instance_std_array.bin', function(r) {
    assert.equal(r.ofs, 0x10)
    assert.equal(r.qtyEntries, 3)
    assert.equal(r.entrySize, 4)

    assert.deepEqual(r.entries, [
        new Uint8Array([0x11, 0x11, 0x11, 0x11]),
        new Uint8Array([0x22, 0x22, 0x22, 0x22]),
        new Uint8Array([0x33, 0x33, 0x33, 0x33]),
    ])
});
