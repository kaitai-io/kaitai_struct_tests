<?php
namespace Kaitai\Struct\Tests;

class ImportsAbsRelTest extends TestCase {
    public function testImportsAbsRel() {
        $r = ImportsAbsRel::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        $this->assertSame(0x50, $r->one);
        $this->assertSame(0x41, $r->two->one);
        $this->assertSame(0x43, $r->two->two->one);
    }
}
