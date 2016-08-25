var assert = require('assert');
var testHelper = require('testHelper');

testHelper('InstanceStdArray', 'src/instance_std_array.bin', function(r) {
    assert.equal(r.ofs, 0x10);
    assert.equal(r.qtyEntries, 3);
    assert.equal(r.entrySize, 4);

    assert.equal(r.entries.length, 3);
    assert.equal(r.entries[0].toString(), new Uint8Array([0x11, 0x11, 0x11, 0x11]).toString());
    assert.equal(r.entries[1].toString(), new Uint8Array([0x22, 0x22, 0x22, 0x22]).toString());
    assert.equal(r.entries[2].toString(), new Uint8Array([0x33, 0x33, 0x33, 0x33]).toString());
});
