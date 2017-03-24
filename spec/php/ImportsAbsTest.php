<?php
namespace Kaitai\Struct\Tests;

class ImportsAbsTest extends TestCase {
    public function testImportsAbs() {
        $r = ImportsAbs::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        $this->assertEquals(80, $r->len->value);
        $this->assertEquals(80, strlen($r->body));
    }
}
