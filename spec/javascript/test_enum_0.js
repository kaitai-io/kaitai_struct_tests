var assert = require('assert');
var testHelper = require('testHelper');

testHelper('Enum0', 'src/enum_0.bin', function(r, Enum0) {
    assert.equal(r.pet1, Enum0.Animal.CAT);
    assert.equal(r.pet2, Enum0.Animal.CHICKEN);
});
