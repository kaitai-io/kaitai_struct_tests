<?php
namespace Kaitai\Struct\Tests;

class OptionalIdTest extends TestCase {
    public function testOptionalId() {
        $r = OptionalId::fromFile(self::SRC_DIR_PATH . '/fixed_struct.bin');

        $this->assertSame(80, $r->_unnamed0());
        $this->assertSame(65, $r->_unnamed1());
        $this->assertSame("\x43\x4B\x2D\x31\xFF", $r->_unnamed2());
    }
}
