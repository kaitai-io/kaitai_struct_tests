<?php
namespace Kaitai\Struct\Tests;

class Expr1Test extends TestCase {
    public function testExpr1() {
        $r = Expr1::fromFile(self::SRC_DIR_PATH . "/str_encodings.bin");

        $this->assertEquals($r->lenOf1(), 10);
        $this->assertEquals($r->lenOf1Mod(), 8);
        $this->assertEquals($r->str1(), "Some ASC");
        $this->assertEquals($r->str1Len(), 8);
    }
}
