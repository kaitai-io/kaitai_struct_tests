<?php
namespace Kaitai\Struct\Tests;

class RecursiveOneTest extends TestCase {
    public function testRecursiveOne() {
        $r = RecursiveOne::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        $this->assertEquals(0x50, $r->one);
        $this->assertEquals(0x41, $r->next->one);
        $this->assertEquals(0x43, $r->next->next->one);
        $this->assertEquals(0x2d4b, $r->next->next->next->finisher);
    }
}
