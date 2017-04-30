var assert = require('assert');
var testHelper = require('testHelper');

testHelper('FloatToI', 'src/floating_points.bin', function(r) {
  assert.equal(r.singleValue, 0.5);
  assert.equal(r.doubleValue, 0.25);
  
  assert.equal(r.singleI, 0);
  assert.equal(r.doubleI, 0);
  assert.equal(r.float1I, 1);
  assert.equal(r.float2I, 1);
  assert.equal(r.float3I, 1);
  assert.equal(r.float4I, -2);
});
