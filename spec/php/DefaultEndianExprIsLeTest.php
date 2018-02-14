<?php
namespace Kaitai\Struct\Tests;

class DefaultEndianExprIsLeTest extends TestCase {
    public function testDefaultEndianExprIsLe() {
        $r = DefaultEndianExprIsLe::fromFile(self::SRC_DIR_PATH . "/endian_expr.bin");

        $this->assertEquals("II", $r->docs[0]->indicator);
        $this->assertEquals(0x42, $r->docs[0]->main->someInt);
        $this->assertEquals(0x42, $r->docs[0]->main->someIntBe);
        $this->assertEquals(0x42, $r->docs[0]->main->someIntLe);

        $this->assertEquals("MM", $r->docs[1]->indicator);
        $this->assertEquals(0x42, $r->docs[1]->main->someInt);
        $this->assertEquals(0x42, $r->docs[1]->main->someIntBe);
        $this->assertEquals(0x42, $r->docs[1]->main->someIntLe);

        $this->assertEquals("XX", $r->docs[2]->indicator);
        $this->assertEquals(0x42, $r->docs[2]->main->someInt);
        $this->assertEquals(0x42, $r->docs[2]->main->someIntBe);
        $this->assertEquals(0x42, $r->docs[2]->main->someIntLe);
    }
}
