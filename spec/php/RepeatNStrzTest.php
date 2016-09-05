<?php
namespace Kaitai\Struct\Tests;

class RepeatNStrzTest extends TestCase {
    public function testRepeatNStrz() {
        $r = RepeatNStrz::fromFile(self::SRC_DIR_PATH . "/repeat_n_strz.bin");

        $this->assertEquals(2, $r->qty(), 2);
        $this->assertEquals(["foo", "bar"], $r->lines());
    }
}
