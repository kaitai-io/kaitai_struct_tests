var assert = require('assert');
var testHelper = require('testHelper');

testHelper('Expr0', 'src/str_encodings.bin', function(r) {
    assert.equal(r.mustBeF7, 0xf7);
    assert.equal(r.mustBeAbc123, "abc123");
});
