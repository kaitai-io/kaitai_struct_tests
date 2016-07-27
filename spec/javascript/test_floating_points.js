var assert = require('assert');
var testHelper = require('testHelper');

testHelper('FloatingPoints', 'src/floating_points.bin', function(r) {
    var delta = 1e-6;
    
    assert.equal(r.singleValue, 0.5);
    assert.equal(r.singleValueBe, 0.5);
    assert.equal(r.doubleValue, 0.25);
    assert.equal(r.doubleValueBe, 0.25);

    assert(Math.abs(r.approximateValue - 1.2345) < delta);

    assert(Math.abs(r.singleValuePlusInt - 1.5) < delta);
    assert(Math.abs(r.singleValuePlusFloat - 1.0) < delta);
    assert(Math.abs(r.doubleValuePlusFloat - 0.3) < delta);
});
