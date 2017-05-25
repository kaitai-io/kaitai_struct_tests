<?php
namespace Kaitai\Struct\Tests;

class DefaultEndianExprInheritedTest extends TestCase {
    public function testDefaultEndianExprInherited() {
        $r = DefaultEndianExprInherited::fromFile(self::SRC_DIR_PATH . "/endian_expr.bin");

        $this->assertEquals("II", $r->docs[0]->indicator);
        $this->assertEquals(0x42, $r->docs[0]->main->insides->someInt);
        $this->assertEquals(0x4200, $r->docs[0]->main->insides->more->someInt1);
        $this->assertEquals(0x42, $r->docs[0]->main->insides->more->someInt2);
        $this->assertEquals(0x42, $r->docs[0]->main->insides->more->someInst);

        $this->assertEquals("MM", $r->docs[1]->indicator);
        $this->assertEquals(0x42, $r->docs[1]->main->insides->someInt);
        $this->assertEquals(0x42, $r->docs[1]->main->insides->more->someInt1);
        $this->assertEquals(0x4200, $r->docs[1]->main->insides->more->someInt2);
        $this->assertEquals(0x42000000, $r->docs[1]->main->insides->more->someInst);

        $this->assertEquals("XX", $r->docs[2]->indicator);
        $this->assertEquals(0x42, $r->docs[2]->main->insides->someInt);
        $this->assertEquals(0x42, $r->docs[2]->main->insides->more->someInt1);
        $this->assertEquals(0x4200, $r->docs[2]->main->insides->more->someInt2);
        $this->assertEquals(0x42000000, $r->docs[2]->main->insides->more->someInst);
    }
}
