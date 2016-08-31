<?php
namespace Kaitai\Struct\Tests;

class Expr0Test extends TestCase {
    public function testExpr0() {
        $r = Expr0::fromFile(self::SRC_DIR_PATH . "/str_encodings.bin");

        $this->assertEquals(0xf7, $r->mustBeF7);
        $this->assertEquals("abc123", $r->mustBeAbc123);
    }
}
