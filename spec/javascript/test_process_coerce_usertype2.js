var assert = require('assert');
var testHelper = require('testHelper');

testHelper('ProcessCoerceUsertype2', 'src/process_coerce_bytes.bin', function(r) {
  assert.equal(r.records[0].flag, 0);
  assert.equal(r.records[0].buf.value, 0x41414141);
  assert.equal(r.records[1].flag, 1);
  assert.equal(r.records[1].buf.value, 0x42424242);
});
