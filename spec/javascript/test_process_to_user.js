var assert = require('assert');
var testHelper = require('testHelper');

testHelper('ProcessToUser', 'src/process_rotate.bin', function(r) {
    assert.equal(r.buf1.str, "Hello");
});
