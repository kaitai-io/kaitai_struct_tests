var assert = require('assert');
var testHelper = require('testHelper');

testHelper('TermStrz', 'src/term_strz.bin', function(r) {
    assert.equal(r.s1, "foo");
    assert.equal(r.s2, "bar");
    assert.equal(r.s3, "|baz@");
})
