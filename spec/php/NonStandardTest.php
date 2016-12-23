<?php
namespace Kaitai\Struct\Tests;

class NonStandardTest extends TestCase {
    public function testNonStandard() {
        $r = NonStandard::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        $this->assertEquals(0x50, $r->foo);
    }
}
