var assert = require('assert');
var testHelper = require('testHelper');

testHelper('RepeatUntilComplex', 'src/repeat_until_complex.bin', function(r) {
  assert.equal(r.first.length, 3);
  assert.equal(r.first[0].count, 4);
  assert.deepEqual(r.first[0].values, [1, 2, 3, 4]);
  assert.equal(r.first[1].count, 2);
  assert.deepEqual(r.first[1].values, [1, 2]);
  assert.equal(r.first[2].count, 0);
  assert.deepEqual(r.first[2].values, []);

  assert.equal(r.second.length, 4);
  assert.equal(r.second[0].count, 6);
  assert.deepEqual(r.second[0].values, [1, 2, 3, 4, 5, 6]);
  assert.equal(r.second[1].count, 3);
  assert.deepEqual(r.second[1].values, [1, 2, 3]);
  assert.equal(r.second[2].count, 4);
  assert.deepEqual(r.second[2].values, [1, 2, 3, 4]);
  assert.equal(r.second[3].count, 0);
  assert.deepEqual(r.second[3].values, []);
  
  assert.deepEqual(r.third, [102, 111, 111, 98, 97, 114, 0]);
});
