<?php
namespace Kaitai\Struct\Tests;

class ProcessXor4ValueTest extends TestCase {
    public function testProcessXor4Value() {
        $r = ProcessXor4Value::fromFile(self::SRC_DIR_PATH . "/process_xor_4.bin");

        $this->assertEquals("\xec\xbb\xa3\x14", $r->key());
        $this->assertEquals("foo bar", $r->buf());
    }
}
