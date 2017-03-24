<?php
namespace Kaitai\Struct\Tests;

class Imports0Test extends TestCase {
    public function testImports0() {
        $r = Imports0::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        $this->assertEquals(0x50, $r->two);
        $this->assertEquals(0x41, $r->hw->one);
        $this->assertEquals(0x41, $r->hwOne);
    }
}
