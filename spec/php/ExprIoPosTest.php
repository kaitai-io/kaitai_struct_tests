<?php
namespace Kaitai\Struct\Tests;

class ExprIoPosTest extends TestCase {
    public function testExprIoPos() {
        $r = ExprIoPos::fromFile(self::SRC_DIR_PATH . "/expr_io_pos.bin");

        $this->assertEquals('CURIOSITY', $r->substream1->myStr);
        $this->assertEquals("\x11\x22\x33\x44", $r->substream1->body);
        $this->assertEquals(0x42, $r->substream1->number);

        $this->assertEquals('KILLED', $r->substream2->myStr);
        $this->assertEquals('a cat', $r->substream2->body);
        $this->assertEquals(0x67, $r->substream2->number);
    }
}
