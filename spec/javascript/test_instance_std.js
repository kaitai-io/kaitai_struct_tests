var assert = require('assert');
var testHelper = require('testHelper');

testHelper('InstanceStd', 'src/str_encodings.bin', function(r) {
    assert.equal(r.header, "Some ");
});
