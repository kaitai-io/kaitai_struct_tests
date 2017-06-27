<?php
namespace Kaitai\Struct\Tests;

class IndexSizesTest extends TestCase {
    public function testIndexSizes() {
        $r = IndexSizes::fromFile(self::SRC_DIR_PATH . "/index_sizes.bin");

        $this->assertEquals(3, $r->qty);

        $this->assertEquals(1, $r->sizes[0]);
        $this->assertEquals(8, $r->sizes[1]);
        $this->assertEquals(4, $r->sizes[2]);

        $this->assertEquals("A", $r->bufs[0]);
        $this->assertEquals("BBBBBBBB", $r->bufs[1]);
        $this->assertEquals("CCCC", $r->bufs[2]);
    }
}
