<?php
namespace Kaitai\Struct\Tests;

class Expr3Test extends TestCase {
    public function testExpr3() {
        $r = Expr3::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        $this->assertEquals(80, $r->one);
        $this->assertEquals('ACK', $r->two);

        $this->assertEquals('@ACK', $r->three);
        $this->assertEquals('_ACK_', $r->four);
        $this->assertEquals(true, $r->isStrEq);
        $this->assertEquals(false, $r->isStrNe);
        $this->assertEquals(true, $r->isStrLt);
        $this->assertEquals(false, $r->isStrGt);
        $this->assertEquals(true, $r->isStrLe);
        $this->assertEquals(false, $r->isStrGe);
        $this->assertEquals(true, $r->isStrLt2);
        $this->assertEquals(true, $r->testNot);
    }
}
