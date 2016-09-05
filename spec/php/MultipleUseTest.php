<?php
namespace Kaitai\Struct\Tests;

class MultipleUseTest extends TestCase {
    public function testMultipleUse() {
        $r = MultipleUse::fromFile(self::SRC_DIR_PATH . "/position_abs.bin");

        $this->assertEquals(0x20, $r->t1()->firstUse()->value());
        $this->assertEquals(0x20, $r->t2()->secondUse()->value());
    }
}
