<?php
namespace Kaitai\Struct\Tests;

class EosExceptionU4Test extends TestCase {
    /**
     * @expectedException \RuntimeException
     * @expectedExceptionMessage Requested 4 bytes, but got only 3 bytes
     */
    public function testEosExceptionU4() {
        EosExceptionU4::fromFile(self::SRC_DIR_PATH . "/term_strz.bin");
    }
}
