var assert = require('assert');
var testHelper = require('testHelper');

testHelper('TsPacketHeader', 'src/ts_packet.bin', function(r, TsPacketHeader) {
  assert.equal(r.syncByte, 0x47);
  assert.equal(r.transportErrorIndicator, false);
  assert.equal(r.payloadUnitStartIndicator, false);
  assert.equal(r.transportPriority, true);
  assert.equal(r.pid, 33);
  assert.equal(r.transportScramblingControl, 0);
  assert.equal(r.adaptationFieldControl, TsPacketHeader.AdaptationFieldControlEnum.PAYLOAD_ONLY);
});
