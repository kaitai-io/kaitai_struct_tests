<?php
namespace Kaitai\Struct\Tests;

class ExprBytesCmpTest extends TestCase {
    public function testExprBytesCmp() {
        $r = ExprBytesCmp::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        $this->assertEquals('P', $r->one);
        $this->assertEquals('ACK', $r->two);

        $this->assertEquals(true, $r->isEq);
        $this->assertEquals(false, $r->isNe);
        $this->assertEquals(true, $r->isLt);
        $this->assertEquals(false, $r->isGt);
        $this->assertEquals(true, $r->isLe);
        $this->assertEquals(false, $r->isGe);
        $this->assertEquals(false, $r->isLt2);
        $this->assertEquals(true, $r->isGt2);
    }
}
