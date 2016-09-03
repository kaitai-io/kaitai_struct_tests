<?php
namespace Kaitai\Struct\Tests;

class RepeatNStructTest extends TestCase {
    public function testRepeatNStruct() {
        $r = RepeatNStruct::fromFile(self::SRC_DIR_PATH . "/repeat_n_struct.bin");

        $this->assertEquals(2, $r->qty());
        $this->assertEquals(0x10, $r->chunks()->get(0)->offset());
        $this->assertEquals(0x2078,$r->chunks()->get(0)->len());
        $this->assertEquals(0x2088,$r->chunks()->get(1)->offset());
        $this->assertEquals(0xf, $r->chunks()->get(1)->len());
    }
}
