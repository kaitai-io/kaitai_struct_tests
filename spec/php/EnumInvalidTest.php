<?php
namespace Kaitai\Struct\Tests;

class EnumInvalidTest extends TestCase {
    public function testEnumInvalid() {
        $r = EnumInvalid::fromFile(self::SRC_DIR_PATH . "/term_strz.bin");
        $this->assertSame($r->pet1(), EnumInvalid\Animal::DOG);
        $this->assertSame($r->pet2(), 0x6f);
    }
}
