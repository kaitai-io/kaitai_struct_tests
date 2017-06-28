<?php
namespace Kaitai\Struct\Tests;

class EnumFancyTest extends TestCase {
    public function testEnumFancy() {
        $r = EnumFancy::fromFile(self::SRC_DIR_PATH . "/enum_0.bin");

        $this->assertEquals($r->pet1(), EnumFancy\Animal::CAT);
        $this->assertEquals($r->pet2(), EnumFancy\Animal::CHICKEN);
    }
}
