<?php
namespace Kaitai\Struct\Tests;

class RepeatNStrzDoubleTest extends TestCase {
    public function testRepeatNStrzDouble() {
        $r = RepeatNStrzDouble::fromFile(self::SRC_DIR_PATH . "/repeat_n_strz.bin");

        $this->assertEquals(2, $r->qty);
        $this->assertEquals(['foo'], $r->lines1);
        $this->assertEquals(['bar'], $r->lines2);
    }
}
