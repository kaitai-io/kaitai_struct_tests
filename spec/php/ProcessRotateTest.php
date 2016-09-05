<?php
namespace Kaitai\Struct\Tests;

class ProcessRotateTest extends TestCase {
    public function testProcessRotate() {
        $r = ProcessRotate::fromFile(self::SRC_DIR_PATH . "/process_rotate.bin");

        $this->assertEquals("Hello", $r->buf1());
        $this->assertEquals("World", $r->buf2());
        $this->assertEquals("There", $r->buf3());
    }
}
