<?php
namespace Kaitai\Struct\Tests;

class ExprEnumTest extends TestCase {
    public function testExprEnum() {
        $r = ExprEnum::fromFile(self::SRC_DIR_PATH . "/term_strz.bin");

        $this->assertEquals(ExprEnum\Animal::DOG, $r->constDog);
        $this->assertEquals(ExprEnum\Animal::BOOM, $r->derivedBoom);
        $this->assertEquals(ExprEnum\Animal::DOG, $r->derivedDog);
    }
}
