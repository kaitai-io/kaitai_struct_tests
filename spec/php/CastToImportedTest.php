<?php
namespace Kaitai\Struct\Tests;

class CastToImportedTest extends TestCase {
    public function testCastToImported() {
        $r = CastToImported::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        $this->assertEquals(0x50, $r->one->one);
        $this->assertEquals(0x50, $r->oneCasted->one);
    }
}
