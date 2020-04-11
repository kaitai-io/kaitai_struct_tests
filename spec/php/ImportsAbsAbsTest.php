<?php
namespace Kaitai\Struct\Tests;

class ImportsAbsAbsTest extends TestCase {
    public function testImportsAbsAbs() {
        $r = ImportsAbsAbs::fromFile(self::SRC_DIR_PATH . '/fixed_struct.bin');

        $this->assertSame(80, $r->one());
        $this->assertSame(65, $r->two()->one());
        $this->assertSame(67, $r->two()->two()->one());
    }
}
