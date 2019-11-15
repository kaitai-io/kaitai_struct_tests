<?php
namespace Kaitai\Struct\Tests;

class SwitchRepeatExprTest extends TestCase {
    public function testSwitchRepeatExpr() {
        $r = SwitchRepeatExpr::fromFile(self::SRC_DIR_PATH . "/switch_tlv.bin");

        $this->assertEquals(0x11, $r->code);
        $this->assertEquals(9, $r->size);
        $this->assertEquals("Stuff\0Me\0", $r->body[0]->first);
    }
}
