var assert = require('assert');
var testHelper = require('testHelper');

testHelper('ProcessCoerceBytes', 'src/process_coerce_bytes.bin', function(r) {
  assert.equal(r.records[0].flag, 0);
  assert.equal(KaitaiStream.bytesToStr(r.records[0].buf, "UTF-8"), "AAAA");
  assert.equal(r.records[1].flag, 1);
  assert.equal(KaitaiStream.bytesToStr(r.records[1].buf, "UTF-8"), "BBBB");
});
