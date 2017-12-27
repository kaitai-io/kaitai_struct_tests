<?php
namespace Kaitai\Struct\Tests;

class YamlIntsTest extends TestCase {
    public function testYamlInts() {
        $r = YamlInts::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        $this->assertEquals(0xffffffff, $r->testU4Dec);
        $this->assertEquals(0xffffffff, $r->testU4Hex);
        $this->assertEquals(0xffffffffffffffff, $r->testU8Dec);
        $this->assertEquals(0xffffffffffffffff, $r->testU8Hex);
    }
}
