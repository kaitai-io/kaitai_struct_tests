<?php
namespace Kaitai\Struct\Tests;

class StrLiterals2Test extends TestCase {
    public function testStrLiterals2() {
        $r = StrLiterals2::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        $this->assertEquals('$foo', $r->dollar1);
        $this->assertEquals('${foo}', $r->dollar2);
        $this->assertEquals('#{foo}', $r->hash);
        $this->assertEquals('@foo', $r->atSign);
    }
}
