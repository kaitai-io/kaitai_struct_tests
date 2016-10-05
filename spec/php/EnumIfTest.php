<?php
namespace Kaitai\Struct\Tests;

class EnumIfTest extends TestCase {
    public function testEnumIf() {
        $r = EnumIf::fromFile(self::SRC_DIR_PATH . "/if_struct.bin");

        $this->assertEquals($r->op1()->opcode(), EnumIf\Opcodes::A_STRING);
        $this->assertEquals("foo", $r->op1()->argStr()->str());

        $this->assertEquals($r->op2()->opcode(), EnumIf\Opcodes::A_TUPLE);
        $this->assertEquals(0x42, $r->op2()->argTuple()->num1());
        $this->assertEquals(0x43, $r->op2()->argTuple()->num2());

        $this->assertEquals($r->op3()->opcode(), EnumIf\Opcodes::A_STRING);
        $this->assertEquals("bar", $r->op3()->argStr()->str());
    }
}
