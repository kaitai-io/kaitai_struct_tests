<?php
namespace Kaitai\Struct\Tests;

class SwitchMultiBoolOpsTest extends TestCase {
    public function testSwitchMultiBoolOps() {
        $r = SwitchMultiBoolOps::fromFile(self::SRC_DIR_PATH . "/switch_integers.bin");

        $this->assertEquals(4, count($r->opcodes));

        $this->assertEquals(1, $r->opcodes[0]->code);
        $this->assertEquals(7, $r->opcodes[0]->body);

        $this->assertEquals(2, $r->opcodes[1]->code);
        $this->assertEquals(0x4040, $r->opcodes[1]->body);

        $this->assertEquals(4, $r->opcodes[2]->code);
        $this->assertEquals(4919, $r->opcodes[2]->body);

        $this->assertEquals(8, $r->opcodes[3]->code);
        $this->assertEquals(4919, $r->opcodes[3]->body);
    }
}
