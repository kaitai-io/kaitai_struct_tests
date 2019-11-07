<?php
namespace Kaitai\Struct\Tests;

class EosExceptionBytesTest extends TestCase {
    /**
     * @expectedException \RuntimeException
     * @expectedExceptionMessage Requested 13 bytes, but got only 12 bytes
     */
    public function testEosExceptionBytes() {
        EosExceptionBytes::fromFile(self::SRC_DIR_PATH . "/term_strz.bin");
    }
}
