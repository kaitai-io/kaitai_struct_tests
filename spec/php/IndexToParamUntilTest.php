<?php
namespace Kaitai\Struct\Tests;

class IndexToParamUntilTest extends TestCase {
    public function testIndexToParamUntil() {
        $r = IndexToParamUntil::fromFile(self::SRC_DIR_PATH . "/index_sizes.bin");

        $this->assertEquals(3, $r->qty);

        $this->assertEquals(1, $r->sizes[0]);
        $this->assertEquals(8, $r->sizes[1]);
        $this->assertEquals(4, $r->sizes[2]);

        $this->assertEquals("A", $r->blocks[0]->buf);
        $this->assertEquals("BBBBBBBB", $r->blocks[1]->buf);
        $this->assertEquals("CCCC", $r->blocks[2]->buf);
    }
}
