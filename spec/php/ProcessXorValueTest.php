<?php
namespace Kaitai\Struct\Tests;

class ProcessXorValueTest extends TestCase {
    public function testProcessXorValue() {
        $r = ProcessXorValue::fromFile(self::SRC_DIR_PATH . "/process_xor_1.bin");

        $this->assertEquals(0xff, $r->key());
        $this->assertEquals("foo bar", $r->buf());
    }
}
