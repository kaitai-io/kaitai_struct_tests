<?php
namespace Kaitai\Struct\Tests;

class YamlIntsTest extends TestCase {
    public function testYamlInts() {
        $r = YamlInts::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        $this->assertSame(0xffffffff, $r->testU4Dec);
        $this->assertSame(0xffffffff, $r->testU4Hex);
        $this->assertSame(0xffffffffffffffff, $r->testU8Dec);
        $this->assertSame(0xffffffffffffffff, $r->testU8Hex);
    }
}
