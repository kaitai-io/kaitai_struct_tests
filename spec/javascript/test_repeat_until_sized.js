var assert = require('assert');
var testHelper = require('testHelper');

testHelper('RepeatUntilSized', 'src/repeat_until_process.bin', function(r) {
  assert.equal(r.records.length, 3);

  assert.equal(r.records[0].marker, 0xe8);
  assert.equal(r.records[0].body, 0xaaaaaaba);

  assert.equal(r.records[1].marker, 0xfa);
  assert.equal(r.records[1].body, 0xaaaab89e);

  assert.equal(r.records[2].marker, 0xaa);
  assert.equal(r.records[2].body, 0x55555555);
});
