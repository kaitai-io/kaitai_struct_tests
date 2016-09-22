<?php
namespace Kaitai\Struct\Tests;

class NestedSameName2Test extends TestCase {
    public function testNestedSameName2() {
        $r = NestedSameName2::fromFile(self::SRC_DIR_PATH . "/nested_same_name2.bin");

        $this->assertEquals(0x42, $r->version);
        $this->assertEquals(2, $r->mainData->mainSize);
        $this->assertEquals("\x11\x11\x11\x11", $r->mainData->foo->data1);
        $this->assertEquals(3, $r->dummy->dummySize);
        $this->assertEquals("\x22\x22\x22\x22\x22\x22", $r->dummy->foo->data2);
    }
}
