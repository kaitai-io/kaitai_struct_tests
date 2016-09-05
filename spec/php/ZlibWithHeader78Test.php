<?php
namespace Kaitai\Struct\Tests;

class ZlibWithHeader78Test extends TestCase {
    public function testZlibWithHeader78() {
        $r = ZlibWithHeader78::fromFile(self::SRC_DIR_PATH . "/zlib_with_header_78.bin");
        $this->assertEquals("a quick brown fox jumps over", $r->data());
    }
}
