<?php
namespace Kaitai\Struct\Tests;

class IfValuesTest extends TestCase {
    public function testIfValues() {
        $r = IfValues::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        $this->assertEquals(80, $r->codes[0]->opcode);
        $this->assertEquals(40, $r->codes[0]->halfOpcode);
        $this->assertEquals(65, $r->codes[1]->opcode);
        $this->assertEquals(null, $r->codes[1]->halfOpcode);
        $this->assertEquals(67, $r->codes[2]->opcode);
        $this->assertEquals(null, $r->codes[2]->halfOpcode);
    }
}
