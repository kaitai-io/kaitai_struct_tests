<?php
namespace Kaitai\Struct\Tests;

class RepeatEosStructTest extends TestCase {
    public function testRepeatEosStruct() {
        $r = RepeatEosStruct::fromFile(self::SRC_DIR_PATH . "/repeat_eos_struct.bin");

        $this->assertEquals(2, count($r->chunks));
        $this->assertEquals(0, $r->chunks[0]->offset);
        $this->assertEquals(0x42, $r->chunks[0]->len);
        $this->assertEquals(0x42, $r->chunks[1]->offset);
        $this->assertEquals(0x815, $r->chunks[1]->len);
    }
}
