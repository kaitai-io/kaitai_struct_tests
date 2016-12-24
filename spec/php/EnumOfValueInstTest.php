<?php
namespace Kaitai\Struct\Tests;

class EnumOfValueInstTest extends TestCase {
    public function testEnumOfValueInst() {
        $r = EnumOfValueInst::fromFile(self::SRC_DIR_PATH . "/enum_0.bin");

        $this->assertEquals(EnumOfValueInst\Animal::CAT, $r->pet1);
        $this->assertEquals(EnumOfValueInst\Animal::CHICKEN, $r->pet2);
        $this->assertEquals(EnumOfValueInst\Animal::DOG, $r->pet3);
        $this->assertEquals(EnumOfValueInst\Animal::DOG, $r->pet4);
    }
}
