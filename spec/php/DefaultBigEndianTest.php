<?php
namespace Kaitai\Struct\Tests;

class DefaultBigEndianTest extends TestCase {
    public function testDefaultBigEndian() {
        $r = DefaultBigEndian::fromFile(self::SRC_DIR_PATH . "/enum_0.bin");
        $this->assertEquals(0x7000000, $r->one());
    }
}
