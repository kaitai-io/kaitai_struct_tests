<?php
namespace Kaitai\Struct\Tests;

class EosExceptionBytesTest extends TestCase {
    /**
     * @expectedException \RuntimeException
     * @expectedExceptionMessage Requested 7 bytes, but got only 6 bytes
     */
    public function testEosExceptionBytes() {
        EosExceptionBytes::fromFile(self::SRC_DIR_PATH . "/term_strz.bin");
    }
}
