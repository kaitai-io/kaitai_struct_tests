<?php
namespace Kaitai\Struct\Tests;

class BytesPadTermTest extends TestCase {
    public function testBytesPadTerm() {
        $r = BytesPadTerm::fromFile(self::SRC_DIR_PATH . "/str_pad_term.bin");

        $this->assertEquals('str1', $r->strPad);
        $this->assertEquals('str2foo', $r->strTerm);
        $this->assertEquals('str+++3bar+++', $r->strTermAndPad);
        $this->assertEquals('str4baz@', $r->strTermInclude);
    }
}
