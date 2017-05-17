<?php
namespace Kaitai\Struct\Tests;

class SwitchBytearrayTest extends TestCase {
    public function testSwitchBytearray() {
        $r = SwitchBytearray::fromFile(self::SRC_DIR_PATH . "/switch_opcodes.bin");

        $this->assertEquals(4, count($r->opcodes));

        $this->assertEquals("S", $r->opcodes[0]->code);
        $this->assertEquals('foobar', $r->opcodes[0]->body->value);

        $this->assertEquals("I", $r->opcodes[1]->code);
        $this->assertEquals(0x42, $r->opcodes[1]->body->value);

        $this->assertEquals("I", $r->opcodes[2]->code);
        $this->assertEquals(0x37, $r->opcodes[2]->body->value);

        $this->assertEquals("S", $r->opcodes[3]->code);
        $this->assertEquals('', $r->opcodes[3]->body->value);
    }
}
