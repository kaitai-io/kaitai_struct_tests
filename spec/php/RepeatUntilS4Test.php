<?php
namespace Kaitai\Struct\Tests;

class RepeatUntilS4Test extends TestCase {
    public function testRepeatUntilS4() {
        $r = RepeatUntilS4::fromFile(self::SRC_DIR_PATH . "/repeat_until_s4.bin");

        $this->assertEquals([0x42, 0x1337, -251658241, -1], $r->entries);
        $this->assertEquals("foobar", $r->afterall);
    }
}
