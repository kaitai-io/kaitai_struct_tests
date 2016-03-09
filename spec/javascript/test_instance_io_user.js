var assert = require('assert');
var testHelper = require('testHelper');

testHelper('InstanceIoUser', 'src/instance_io.bin', function(r) {
    assert.equal(r.qtyEntries, 3);

    assert.equal(r.entries[0].name, "the");
    assert.equal(r.entries[1].name, "rainy");
    assert.equal(r.entries[2].name, "day it is");
});
