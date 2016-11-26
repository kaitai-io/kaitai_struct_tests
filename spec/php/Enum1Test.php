<?php
namespace Kaitai\Struct\Tests;

class Enum1Test extends TestCase {
    public function testEnum1() {
        $r = Enum1::fromFile(self::SRC_DIR_PATH . "/enum_0.bin");

        $this->assertEquals(Enum1\MainObj\Animal::CAT, $r->main->submain->pet1);
        $this->assertEquals(Enum1\MainObj\Animal::CHICKEN, $r->main->submain->pet2);
    }
}
