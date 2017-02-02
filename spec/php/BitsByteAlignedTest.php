<?php
namespace Kaitai\Struct\Tests;

class BitsByteAlignedTest extends TestCase {
    public function testBitsByteAligned() {
        $r = BitsByteAligned::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        # 50 (6 + 2) = 010100|00
        $this->assertEquals(0b010100, $r->one);
        # 41
        $this->assertEquals(0x41, $r->byte1);
        # 43 (3 + 1 + 4) = 010|0|0011
        $this->assertEquals(0b010, $r->two);
        $this->assertEquals(false, $r->three);
        # 4B
        $this->assertEquals(0x4b, $r->byte2);
        # 2D 31 (14 + 2) = 00101101 001100|01
        $this->assertEquals(0b00101101001100, $r->four);
        # FF
        $this->assertEquals("\xff", $r->byte3);
        # FF
        $this->assertEquals(0xff, $r->fullByte);
        # 50
        $this->assertEquals(0x50, $r->byte4);
    }
}
