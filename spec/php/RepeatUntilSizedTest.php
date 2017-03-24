<?php
namespace Kaitai\Struct\Tests;

class RepeatUntilSizedTest extends TestCase {
    public function testRepeatUntilSized() {
        $r = RepeatUntilSized::fromFile(self::SRC_DIR_PATH . "/repeat_until_process.bin");

        $this->assertEquals(3, count($r->records));

        $this->assertEquals(0xe8, $r->records[0]->marker);
        $this->assertEquals(0xaaaaaaba, $r->records[0]->body);

        $this->assertEquals(0xfa, $r->records[1]->marker);
        $this->assertEquals(0xaaaab89e, $r->records[1]->body);

        $this->assertEquals(0xaa, $r->records[2]->marker);
        $this->assertEquals(0x55555555, $r->records[2]->body);
    }
}
