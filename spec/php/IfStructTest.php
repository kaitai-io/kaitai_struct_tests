<?php
namespace Kaitai\Struct\Tests;

class IfStructTest extends TestCase {
    public function testIfStruct() {
        $r = IfStruct::fromFile(self::SRC_DIR_PATH . "/if_struct.bin");

        $this->assertEquals(0x53, $r->op1()->opcode());
        $this->assertEquals("foo", $r->op1()->argStr()->str());

        $this->assertEquals(0x54, $r->op2()->opcode());
        $this->assertEquals(0x42, $r->op2()->argTuple()->num1());
        $this->assertEquals(0x43, $r->op2()->argTuple()->num2());

        $this->assertEquals(0x53, $r->op3()->opcode());
        $this->assertEquals("bar", $r->op3()->argStr()->str());
    }
}
