<?php
namespace Kaitai\Struct\Tests;

class NestedSameNameTest extends TestCase {
    public function testNestedSameName() {
        $r = NestedSameName::fromFile(self::SRC_DIR_PATH . "/repeat_n_struct.bin");

        $this->assertEquals(2, $r->mainData->mainSize);
        $this->assertEquals("\x10\0\0\0", $r->mainData->foo->data);
    }
}
