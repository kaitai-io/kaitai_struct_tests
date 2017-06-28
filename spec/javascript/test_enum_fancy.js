var assert = require('assert');
var testHelper = require('testHelper');

testHelper('EnumFancy', 'src/enum_0.bin', function(r, EnumFancy) {
    assert.equal(r.pet1, EnumFancy.Animal.CAT);
    assert.equal(r.pet2, EnumFancy.Animal.CHICKEN);
});
