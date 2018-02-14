<?php
namespace Kaitai\Struct\Tests;

class MetaXrefTest extends TestCase {
    public function testMetaXref() {
        $r = MetaXref::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        // no real test here, just parsing should be ok
        $this->markTestSkipped();
    }
}
