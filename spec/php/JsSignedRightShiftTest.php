<?php
namespace Kaitai\Struct\Tests;

class JsSignedRightShiftTest extends TestCase {
    public function testJsSignedRightShift() {
        $r = JsSignedRightShift::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        $this->assertEquals(0x40000000, $r->shouldBe40000000);
        $this->assertEquals(0xa00000, $r->shouldBeA00000);
    }
}
