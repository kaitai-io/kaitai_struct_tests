<?php
namespace Kaitai\Struct\Tests;

class StrLiteralsTest extends TestCase {
    public function testStrLiterals() {
        $r = StrLiterals::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        $this->assertSame("\u{0}\u{1}\u{2}\u{7}\u{8}\u{a}\u{d}\u{9}\u{b}\u{c}\u{1b}\u{3d}\u{7}\u{a}\u{24}\u{263b}", $r->complexStr);
        $this->assertSame("\u{22}\u{22}\u{22}", $r->doubleQuotes);
        $this->assertSame("\u{5c}\u{5c}\u{5c}", $r->backslashes);
        $this->assertSame("\u{0}\u{32}\u{32}", $r->octalEatup);
        $this->assertSame("\u{2}\u{32}", $r->octalEatup2);
    }
}
