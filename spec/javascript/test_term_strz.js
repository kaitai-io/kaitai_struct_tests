var assert = require('assert');
var testHelper = require('testHelper');

testHelper('TermStrz', 'src/term_strz.bin', function(r) {
    assert.equal("foo", r.s1);
    assert.equal("bar", r.s2);
    assert.equal("|baz@", r.s3);
})
