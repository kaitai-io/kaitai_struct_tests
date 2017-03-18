<?php
namespace Kaitai\Struct\Tests;

class NavParentFalse2Test extends TestCase {
    public function testNavParentFalse2() {
        $r = NavParentFalse2::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        $this->assertEquals(80, $r->parentless->foo);
    }
}
