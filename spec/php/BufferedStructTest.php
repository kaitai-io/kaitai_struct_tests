<?php
namespace Kaitai\Struct\Tests;

class BufferedStructTest extends TestCase {
    public function testBufferedStruct() {
        $r = BufferedStruct::fromFile(self::SRC_DIR_PATH . "/buffered_struct.bin");

        $this->assertEquals(0x10, $r->len1());

        $expectedBytes = "\x42\x00\x00\x00"
            . "\x43\x00\x00\x00"
            . "\xff\xff\xff\xff"
            . "\xff\xff\xff\xff";
        $this->assertEquals($expectedBytes, $r->_raw_block1());
        $this->assertEquals(0x42, $r->block1()->number1());
        $this->assertEquals(0x43, $r->block1()->number2());

        $this->assertEquals(0x8, $r->len2());

        $this->assertEquals(
            "\x44\x00\x00\x00"
            . "\x45\x00\x00\x00",
            $r->_raw_block2()
        );
        $this->assertEquals(0x44, $r->block2()->number1());
        $this->assertEquals(0x45, $r->block2()->number2());

        $this->assertEquals(0xee, $r->finisher());
    }
}
