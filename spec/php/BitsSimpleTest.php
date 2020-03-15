<?php
namespace Kaitai\Struct\Tests;

class BitsSimpleTest extends TestCase {
    public function testBitsSimple() {
        $r = BitsSimple::fromFile(self::SRC_DIR_PATH . '/fixed_struct.bin');

        # 50 41
        $this->assertSame(0x50, $r->byte1);
        $this->assertSame(0x41, $r->byte2);

        # 43 (1 + 3 + 4) = 0|100|0011
        $this->assertSame(false, $r->bitsA);
        $this->assertSame(0b100, $r->bitsB);
        $this->assertSame(0b0011, $r->bitsC);

        # 4B 2D 31 (10 + 3 + 11) = 01001011 00|101|101 00110001
        $this->assertSame(0b0100101100, $r->largeBits1);
        $this->assertSame(0b101, $r->spacer);
        $this->assertSame(0b10100110001, $r->largeBits2);

        # FF FF
        $this->assertSame(-1, $r->normalS2);

        # 50 41 43
        $this->assertSame(0x504143, $r->byte8910);

        # 4B 2D 55 2D
        $this->assertSame(0x4B2D552D, $r->byte11To14);

        # 44 45 46 FF FF
        $this->assertSame(0x444546FFFF, $r->byte15To19);

        # FF FF FF FF FF FF FF FF
        # no 64-bit unsigned integers in PHP, so it would be signed => -1
        $this->assertSame(-1, $r->byte20To27);

        $this->assertSame(123, $r->testIfB1);
    }
}
