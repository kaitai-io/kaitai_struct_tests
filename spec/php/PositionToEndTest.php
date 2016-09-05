<?php
namespace Kaitai\Struct\Tests;

class PositionToEndTest extends TestCase {
    public function testPositionToEnd() {
        $r = PositionToEnd::fromFile(self::SRC_DIR_PATH . "/position_to_end.bin");
        $this->assertEquals(0x42, $r->index()->foo());
        $this->assertEquals(0x1234, $r->index()->bar());
    }
}
