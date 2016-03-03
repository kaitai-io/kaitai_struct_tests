var assert = require('assert');
var testHelper = require('testHelper');

testHelper('Expr2', 'src/str_encodings.bin', function(r) {
    assert.equal(r.str1.lenOrig, 10)
    assert.equal(r.str1.lenMod, 8)
    assert.equal(r.str1.str, "Some ASC")

    assert.equal(r.str1Len, 8)
    assert.equal(r.str1LenMod, 8)
    assert.equal(r.str1Byte1, 0x49)
    assert.equal(r.str1Avg, 0x49)
    assert.equal(r.str1Char5, "e")

    assert.equal(r.str1Tuple5.byte1, 0x65)
    assert.equal(r.str1Tuple5.byte2, 0x20)
    assert.equal(r.str1Tuple5.avg, 0x42)

    assert.equal(r.str2Tuple5.byte1, 0x65)
    assert.equal(r.str2Tuple5.byte2, 0x20)
    assert.equal(r.str2Tuple5.avg, 0x42)
});
