<?php
namespace Kaitai\Struct\Tests;

class TsPacketHeaderTest extends TestCase {
    public function testTsPacketHeader() {
        $r = TsPacketHeader::fromFile(self::SRC_DIR_PATH . "/ts_packet.bin");

        $this->assertEquals(0x47, $r->syncByte);
        $this->assertEquals(false, $r->transportErrorIndicator);
        $this->assertEquals(false, $r->payloadUnitStartIndicator);
        $this->assertEquals(true, $r->transportPriority);
        $this->assertEquals(33, $r->pid);
        $this->assertEquals(0, $r->transportScramblingControl);
        $this->assertEquals(TsPacketHeader\AdaptationFieldControlEnum::PAYLOAD_ONLY, $r->adaptationFieldControl);
    }
}
