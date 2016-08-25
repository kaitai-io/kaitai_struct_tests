<?php
namespace Kaitai\Struct\Tests;

class BufferedStructTest extends TestCase {
    public function testBufferedStruct() {
        $r = BufferedStruct::fromFile(self::SRC_DIR_PATH . "/buffered_struct.bin");

        $this->assertEquals($r->len1(), 0x10);

        $expectedBytes = "\x42\x00\x00\x00"
            . "\x43\x00\x00\x00"
            . "\xff\xff\xff\xff"
            . "\xff\xff\xff\xff";
        $this->assertEquals($expectedBytes, $r->_raw_block1());
        $this->assertEquals($r->block1()->number1(), 0x42);
        $this->assertEquals($r->block1()->number2(), 0x43);

        $this->assertEquals($r->len2(), 0x8);

        $this->assertEquals(
            "\x44\x00\x00\x00"
            . "\x45\x00\x00\x00",
            $r->_raw_block2()
        );
        $this->assertEquals($r->block2()->number1(), 0x44);
        $this->assertEquals($r->block2()->number2(), 0x45);

        $this->assertEquals($r->finisher(), 0xee);
    }
}
