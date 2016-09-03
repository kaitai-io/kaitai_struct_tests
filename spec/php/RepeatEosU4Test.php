<?php
namespace Kaitai\Struct\Tests;

class RepeatEosU4Test extends TestCase {
    public function testRepeatEosU4() {
        $r = RepeatEosU4::fromFile(self::SRC_DIR_PATH . "/repeat_eos_struct.bin");
        $this->assertEquals([0, 0x42, 0x42, 0x815], $r->numbers());
    }
}
