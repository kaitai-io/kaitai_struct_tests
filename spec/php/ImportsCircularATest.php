<?php
namespace Kaitai\Struct\Tests;

class ImportsCircularATest extends TestCase {
    public function testImportsCircularA() {
        $r = ImportsCircularA::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        $this->assertSame(0x50, $r->code);
        $this->assertSame(0x41, $r->two->initial);
        $this->assertSame(0x43, $r->two->backRef->code);
        $this->assertSame(0x4b, $r->two->backRef->two->initial);
        $this->assertSame(null, $r->two->backRef->two->backRef);
    }
}
