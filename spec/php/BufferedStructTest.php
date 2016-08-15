<?php
namespace Kaitai\Struct\Tests;

class BufferedStructTest extends \PHPUnit_Framework_TestCase {
    public function testBufferedStruct() {
        $this->markTestIncomplete();
/*
        BufferedStruct r = BufferedStruct::fromFile(self::SRC_DIR_PATH . "buffered_struct.bin");

        $this->assertEquals(r.len1(), 0x10);

        $this->assertEquals(r._raw_block1(), new byte[] {
                0x42, 0, 0, 0,
                0x43, 0, 0, 0,
                -1, -1, -1, -1,
                -1, -1, -1, -1,
        });
        $this->assertEquals(r.block1().number1(), 0x42);
        $this->assertEquals(r.block1().number2(), 0x43);

        $this->assertEquals(r.len2(), 0x8);

        $this->assertEquals(r._raw_block2(), new byte[] {
                0x44, 0, 0, 0,
                0x45, 0, 0, 0,
        });
        $this->assertEquals(r.block2().number1(), 0x44);
        $this->assertEquals(r.block2().number2(), 0x45);

        $this->assertEquals(r.finisher(), 0xee);
*/    
    }
}
