<?php
namespace Kaitai\Struct\Tests;

class EnumToITest extends TestCase {
    public function testEnumToI() {
        $r = EnumToI::fromFile(self::SRC_DIR_PATH . "/enum_0.bin");

        $this->assertEquals(EnumToI\Animal::CAT, $r->pet1());
        $this->assertEquals(EnumToI\Animal::CHICKEN, $r->pet2());

        $this->assertEquals(7, $r->pet1I);
        $this->assertEquals(true, $r->oneLtTwo);
    }
}
