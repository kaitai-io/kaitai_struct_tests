<?php
namespace Kaitai\Struct\Tests;

class PositionAbsTest extends TestCase {
    public function testPositionAbs() {
        $r = PositionAbs::fromFile(self::SRC_DIR_PATH . "/position_abs.bin");

        $this->assertEquals(0x20, $r->indexOffset());
        $this->assertEquals("foo", $r->index()->entry());
    }
}
