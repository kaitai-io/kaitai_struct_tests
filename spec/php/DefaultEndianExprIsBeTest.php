<?php
namespace Kaitai\Struct\Tests;

class DefaultEndianExprIsBeTest extends TestCase {
    public function testDefaultEndianExprIsBe() {
        $r = DefaultEndianExprIsBe::fromFile(self::SRC_DIR_PATH . "/endian_expr.bin");

        // LE
        $this->assertEquals("II", $r->docs[0]->indicator);
        $this->assertEquals(0x42, $r->docs[0]->main->someInt);
        $this->assertEquals(0x42, $r->docs[0]->main->someIntBe);
        $this->assertEquals(0x42, $r->docs[0]->main->someIntLe);

        $this->assertEquals(0x42, $r->docs[0]->main->instInt);
        $this->assertEquals(0x42, $r->docs[0]->main->instSub->foo);

        // BE
        $this->assertEquals("MM", $r->docs[1]->indicator);
        $this->assertEquals(0x42, $r->docs[1]->main->someInt);
        $this->assertEquals(0x42, $r->docs[1]->main->someIntBe);
        $this->assertEquals(0x42, $r->docs[1]->main->someIntLe);

        $this->assertEquals(0x42000000, $r->docs[1]->main->instInt);
        $this->assertEquals(0x42000000, $r->docs[1]->main->instSub->foo);

        // Weird => LE
        $this->assertEquals("XX", $r->docs[2]->indicator);
        $this->assertEquals(0x42000000, $r->docs[2]->main->someInt);
        $this->assertEquals(0x42, $r->docs[2]->main->someIntBe);
        $this->assertEquals(0x42, $r->docs[2]->main->someIntLe);

        $this->assertEquals(0x42, $r->docs[2]->main->instInt);
        $this->assertEquals(0x42, $r->docs[2]->main->instSub->foo);
    }
}
