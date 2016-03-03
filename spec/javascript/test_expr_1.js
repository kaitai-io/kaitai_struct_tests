var assert = require('assert');
var testHelper = require('testHelper');

testHelper('Expr1', 'src/str_encodings.bin', function(r) {
    assert.equal(r.lenOf1, 10);
    assert.equal(r.lenOf1Mod, 8);
    assert.equal(r.str1, "Some ASC");
    assert.equal(r.str1Len, 8);
});
