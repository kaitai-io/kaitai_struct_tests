var assert = require('assert');
var testHelper = require('testHelper');

testHelper('InstanceUserArray', 'src/instance_std_array.bin', function(r) {
    assert.equal(r.ofs, 0x10);
    assert.equal(r.qtyEntries, 3);
    assert.equal(r.entrySize, 4);

    assert.equal(r.userEntries[0].word1, 0x1111);
    assert.equal(r.userEntries[0].word2, 0x1111);
    assert.equal(r.userEntries[1].word1, 0x2222);
    assert.equal(r.userEntries[1].word2, 0x2222);
    assert.equal(r.userEntries[2].word1, 0x3333);
    assert.equal(r.userEntries[2].word2, 0x3333);
});
