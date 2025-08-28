var assert = require('assert');
var testHelper = require('testHelper');

testHelper('ToStringCustom', 'src/term_strz.bin', function(r) {
    assert.strictEqual(r.toString(), "s1 = foo, s2 = bar");
});
