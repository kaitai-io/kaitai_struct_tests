var assert = require('assert');
var testHelper = require('testHelper');

testHelper('FloatingPoints', 'src/floating_points.bin', function(r) {
    var delta = 1e-6;
    
    assert(Math.abs(r.singleValue - 1.2345) < delta);
    assert(Math.abs(r.singleValueBe - 1.2345) < delta);
    assert(Math.abs(r.doubleValue - 123.456) < delta);
    assert(Math.abs(r.doubleValueBe - 123.456) < delta);
    
    assert(Math.abs(r.singleValuePlusInt - 2.2345) < delta);
    assert(Math.abs(r.singleValuePlusFloat - 1.7345) < delta);
    assert(Math.abs(r.doubleValuePlusFloat - 123.506) < delta);
});
