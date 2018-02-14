<?php
namespace Kaitai\Struct\Tests;

class EnumNegativeTest extends TestCase {
    public function testEnumNegative() {
        $r = EnumNegative::fromFile(self::SRC_DIR_PATH . "/enum_negative.bin");

        $this->assertEquals(EnumNegative\Constants::NEGATIVE_ONE, $r->f1);
        $this->assertEquals(EnumNegative\Constants::POSITIVE_ONE, $r->f2);
    }
}
