<?php
namespace Kaitai\Struct\Tests;

class ProcessCustomTest extends TestCase {
    public function testProcessCustom() {
        $r = ProcessCustom::fromFile(self::SRC_DIR_PATH . "/process_rotate.bin");

        $this->assertEquals("\x10\xb3\x94\x94\xf4", $r->buf1);
        $this->assertEquals("\x5f\xba\x7b\x93\x63\x23\x5f", $r->buf2);
        $this->assertEquals("\x29\x33\xb1\x38\xb1", $r->buf3);
    }
}
