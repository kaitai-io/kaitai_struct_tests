<?php
namespace Kaitai\Struct\Tests;

class EofExceptionU4Test extends TestCase {
    /**
     * @expectedException \RuntimeException
     * @expectedExceptionMessage Requested 4 bytes, but got only 3 bytes
     */
    public function testEofExceptionU4() {
        EofExceptionU4::fromFile(self::SRC_DIR_PATH . "/term_strz.bin");
    }
}
