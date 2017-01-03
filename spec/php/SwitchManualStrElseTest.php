<?php
namespace Kaitai\Struct\Tests;

class SwitchManualStrElseTest extends TestCase {
    public function testSwitchManualStrElse() {
        $r = SwitchManualStrElse::fromFile(self::SRC_DIR_PATH . "/switch_opcodes2.bin");

        $this->assertEquals(4, count($r->opcodes));

        $this->assertEquals('S', $r->opcodes[0]->code);
        $this->assertEquals('foo', $r->opcodes[0]->body->value);

        $this->assertEquals('X', $r->opcodes[1]->code);
        $this->assertEquals(0x42, $r->opcodes[1]->body->filler);

        $this->assertEquals('Y', $r->opcodes[2]->code);
        $this->assertEquals(0xcafe, $r->opcodes[2]->body->filler);

        $this->assertEquals('I', $r->opcodes[3]->code);
        $this->assertEquals(7, $r->opcodes[3]->body->value);
    }
}
