<?php
namespace Kaitai\Struct\Tests;

class SwitchManualIntElseTest extends TestCase {
    public function testSwitchManualIntElse() {
        $r = SwitchManualIntElse::fromFile(self::SRC_DIR_PATH . "/switch_opcodes2.bin");

        $this->assertEquals(4, count($r->opcodes));

        $this->assertEquals(83, $r->opcodes[0]->code);
        $this->assertEquals('foo', $r->opcodes[0]->body->value);

        $this->assertEquals(88, $r->opcodes[1]->code);
        $this->assertEquals(0x42, $r->opcodes[1]->body->filler);

        $this->assertEquals(89, $r->opcodes[2]->code);
        $this->assertEquals(0xcafe, $r->opcodes[2]->body->filler);

        $this->assertEquals(73, $r->opcodes[3]->code);
        $this->assertEquals(7, $r->opcodes[3]->body->value);
    }
}
