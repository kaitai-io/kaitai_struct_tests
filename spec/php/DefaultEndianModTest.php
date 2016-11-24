<?php
namespace Kaitai\Struct\Tests;

class DefaultEndianModTest extends TestCase {
    public function testDefaultEndianMod() {
        $r = DefaultEndianMod::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        $this->assertEquals(0x4b434150, $r->main->one);
        $this->assertEquals(-52947, $r->main->nest->two);
        $this->assertEquals(0x5041434b, $r->main->nestBe->two);
    }
}
