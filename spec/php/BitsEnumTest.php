<?php
namespace Kaitai\Struct\Tests;

class BitsEnumTest extends TestCase {
    public function testBitsEnum() {
        $r = BitsEnum::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        // 50 41 (4 + 8 + 1) = 0101|0000 0100|0|001
        $this->assertEquals(BitsEnum\Animal::PLATYPUS, $r->one);
        $this->assertEquals(BitsEnum\Animal::HORSE, $r->two);
        $this->assertEquals(BitsEnum\Animal::CAT, $r->three);
    }
}
