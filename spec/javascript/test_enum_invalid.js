var assert = require('assert');
var testHelper = require('testHelper');

testHelper('EnumInvalid', 'src/term_strz.bin', function(r, EnumInvalid) {
    assert.equal(r.pet1, EnumInvalid.Animal.DOG);
    assert.equal(r.pet2, 0x6f);
});
