<?php
namespace Kaitai\Struct\Tests;

class EnumInvalidTest extends TestCase {
    public function testEnumInvalid() {
        $r = EnumInvalid::fromFile(self::SRC_DIR_PATH . "/term_strz.bin");
        $this->assertEquals($r->pet1(), EnumInvalid\Animal::DOG);
        $this->assertEquals($r->pet2(), 0x6f);
    }
}
