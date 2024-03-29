<?php
// Autogenerated from KST: please remove this line if doing any edits by hand!

namespace Kaitai\Struct\Tests;

class ExprBytesNonLiteralTest extends TestCase {
    public function testExprBytesNonLiteral() {
        $r = ExprBytesNonLiteral::fromFile(self::SRC_DIR_PATH . '/enum_negative.bin');

        $this->assertSame(2, strlen($r->calcBytes()));
        $this->assertSame(255, ord($r->calcBytes()[0]));
        $this->assertSame(1, ord($r->calcBytes()[1]));
    }
}
