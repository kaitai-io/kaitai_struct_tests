var assert = require('assert');
var testHelper = require('testHelper');

testHelper('SwitchManualIntSize', 'src/switch_tlv.bin', function(r) {
  assert.equal(r.chunks.length, 4);

  assert.equal(r.chunks[0].code, 0x11);
  meta = r.chunks[0].body
  assert.equal(meta.title, 'Stuff');
  assert.equal(meta.author, 'Me');

  assert.equal(r.chunks[1].code, 0x22);
  assert.deepEqual(r.chunks[1].body.entries, ['AAAA', 'BBBB', 'CCCC']);

  assert.equal(r.chunks[2].code, 0x33);
  assert.equal(r.chunks[2].body.toString(), [0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80].toString());

  assert.equal(r.chunks[3].code, 0xff);
  assert.equal(r.chunks[3].body, '');
});
