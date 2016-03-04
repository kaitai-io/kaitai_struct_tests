var assert = require('assert');
var testHelper = require('testHelper');

testHelper('NavParent', 'src/nav.bin', function(r) {
    assert.equal(r.header.qtyEntries, 2);
    assert.equal(r.header.filenameLen, 8);

    assert.equal(r.index.entries.length, 2);
    assert.equal(r.index.entries[0].filename, "FIRST___");
    assert.equal(r.index.entries[1].filename, "SECOND__");
});
