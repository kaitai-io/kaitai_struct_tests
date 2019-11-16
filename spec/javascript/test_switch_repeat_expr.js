var assert = require('assert');
var testHelper = require('testHelper');

testHelper('SwitchRepeatExpr', 'src/switch_tlv.bin', function(r) {
  assert.equal(r.code, 0x11);
  assert.equal(r.size, 9);
  assert.equal(KaitaiStream.bytesToStr(r.body[0].first, "UTF-8"), "Stuff\0Me\0");
});
