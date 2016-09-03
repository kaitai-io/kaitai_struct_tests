<?php
namespace Kaitai\Struct\Tests;

class ProcessXor4ConstTest extends TestCase {
    public function testProcessXor4Const() {
        $r = ProcessXor4Const::fromFile(self::SRC_DIR_PATH . "/process_xor_4.bin");
        $this->assertEquals("foo bar", $r->buf());
    }
}
