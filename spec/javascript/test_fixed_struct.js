var assert = require('assert');
var testHelper = require('testHelper');

testHelper('FixedStruct', 'src/fixed_struct.bin', function(r) {
  assert.equal(r.hdr.uint8, 255);
  assert.equal(r.hdr.uint16, 65535);
  assert.equal(r.hdr.uint32, 4294967295);
  assert.equal(r.hdr.uint64, 18446744073709551615);

  assert.equal(r.hdr.sint8, -1);
  assert.equal(r.hdr.sint16, -1);
  assert.equal(r.hdr.sint32, -1);
  assert.equal(r.hdr.sint64, -1);

  assert.equal(r.hdr.uint16le, 66);
  assert.equal(r.hdr.uint32le, 66);
  assert.equal(r.hdr.uint64le, 66);

  assert.equal(r.hdr.sint16le, -66);
  assert.equal(r.hdr.sint32le, -66);
  assert.equal(r.hdr.sint64le, -66);

  assert.equal(r.hdr.uint16be, 66);
  assert.equal(r.hdr.uint32be, 66);
  assert.equal(r.hdr.uint64be, 66);

  assert.equal(r.hdr.sint16be, -66);
  assert.equal(r.hdr.sint32be, -66);
  assert.equal(r.hdr.sint64be, -66);
});
