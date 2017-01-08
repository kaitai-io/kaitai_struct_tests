<?php
namespace Kaitai\Struct\Tests;

class BitsSimpleTest extends TestCase {
    public function testBitsSimple() {
        $r = BitsSimple::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        # 50 41
        $this->assertEquals(0x50, $r->byte1);
        $this->assertEquals(0x41, $r->byte2);

        # 43 (1 + 3 + 4) = 0|100|0011
        $this->assertEquals(0, $r->bitsA);
        $this->assertEquals(0b100, $r->bitsB);
        $this->assertEquals(0b0011, $r->bitsC);

        # 4B 2D 31 (10 + 3 + 11) = 01001011 00|101|101 00110001
        $this->assertEquals(0b0100101100, $r->largeBits1);
        $this->assertEquals(0b101, $r->spacer);
        $this->assertEquals(0b10100110001, $r->largeBits2);

        # FF FF
        $this->assertEquals(-1, $r->normalS2);

        # 50 41 43
        $this->assertEquals(0x504143, $r->byte8910);
    }
}
