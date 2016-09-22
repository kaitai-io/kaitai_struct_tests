<?php
namespace Kaitai\Struct\Tests;

class TypeIntUnaryOpTest extends TestCase {
    public function testTypeIntUnaryOp() {
        $r = TypeIntUnaryOp::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        $this->assertEquals(0x4150, $r->valueS2);
        $this->assertEquals(0x4150ffff312d4b43, $r->valueS8);
        $this->assertEquals(-0x4150, $r->unaryS2);
        $this->assertEquals(-0x4150ffff312d4b43, $r->unaryS8);
    }
}
