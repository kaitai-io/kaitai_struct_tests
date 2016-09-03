<?php
namespace Kaitai\Struct\Tests;

class NavRootTest extends TestCase {
    public function testNavRoot() {
        $r = NavRoot::fromFile(self::SRC_DIR_PATH . "/nav.bin");

        $this->assertEquals(2, $r->header()->qtyEntries());
        $this->assertEquals(8, $r->header()->filenameLen());

        $this->assertEquals(2, $r->index()->entries()->size());
        $this->assertEquals("FIRST___", $r->index()->entries()->get(0)->filename());
        $this->assertEquals("SECOND__", $r->index()->entries()->get(1)->filename());
    }
}
