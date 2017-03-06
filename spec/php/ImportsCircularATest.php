<?php
namespace Kaitai\Struct\Tests;

class ImportsCircularATest extends TestCase {
    public function testImportsCircularA() {
        $r = ImportsCircularA::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        $this->assertEquals(0x50, $r->code);
        $this->assertEquals(0x41, $r->two->initial);
        $this->assertEquals(0x43, $r->two->backRef->code);
        $this->assertEquals(0x4b, $r->two->backRef->two->initial);
        $this->assertEquals(null, $r->two->backRef->two->backRef);
    }
}
