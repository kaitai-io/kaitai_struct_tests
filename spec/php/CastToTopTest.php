<?php
namespace Kaitai\Struct\Tests;

class CastToTopTest extends TestCase {
    public function testCastToTop() {
        $r = CastToTop::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        $this->assertEquals(0x50, $r->code);
        $this->assertEquals(0x41, $r->header->code);
        $this->assertEquals(0x41, $r->headerCasted->code);
    }
}
