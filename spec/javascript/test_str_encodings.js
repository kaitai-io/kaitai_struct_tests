var assert = require('assert');
var testHelper = require('testHelper');

testHelper('StrEncodings', 'src/str_encodings.bin', function(r) {
    assert.equal("Some ASCII", r.str1);
    assert.equal("こんにちは", r.str2);
    assert.equal("こんにちは", r.str3);
    assert.equal("░▒▓", r.str4);
});
