<?php
namespace Kaitai\Struct\Tests;

class ProcessXorConstTest extends TestCase {
    public function testProcessXorConst() {
        $r = ProcessXorConst::fromFile(self::SRC_DIR_PATH . "/process_xor_1.bin");

        $this->assertEquals(0xff, $r->key());
        $this->assertEquals("foo bar", $r->buf());
    }
}
