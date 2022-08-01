var assert = require('assert');
var testHelper = require('testHelper');

testHelper('ToStringCustom', 'src/term_strz.bin', function(r) {
    assert.equal(r.toString(), "s1 = foo, s2 = bar");
});
