<?php
namespace Kaitai\Struct\Tests;

class OptionalIdTest extends TestCase {
    public function testOptionalId() {
        $r = OptionalId::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        $this->assertEquals(0x50, $r->_unnamed0);
        $this->assertEquals(0x41, $r->_unnamed1);
        $this->assertEquals("\x43\x4b\x2d\x31\xff", $r->_unnamed2);
    }
}
