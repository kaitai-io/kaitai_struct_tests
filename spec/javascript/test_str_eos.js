var assert = require('assert');
var testHelper = require('testHelper');

testHelper('StrEos', 'src/term_strz.bin', function(r) {
    assert.equal(r.str, "foo|bar|baz@");
});
