var assert = require('assert');
var testHelper = require('testHelper');

testHelper('Expr0', 'src/str_encodings.bin', function(r) {
    assertEquals(r.mustBeF7(), 0xf7);
    assertEquals(r.mustBeAbc123(), "abc123");
});
