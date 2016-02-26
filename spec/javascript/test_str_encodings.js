var assert = require('assert');
var testHelper = require('testHelper');

testHelper('StrEncodings', 'src/str_encodings.bin', function(r) {
    assert.equal(r.str1, "Some ASCII");
    assert.equal(r.str2, "こんにちは");
    assert.equal(r.str3, "こんにちは");
    assert.equal(r.str4, "░▒▓");
});
