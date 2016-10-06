<?php
namespace Kaitai\Struct\Tests;

class SwitchManualIntSizeEosTest extends TestCase {
    public function testSwitchManualIntSizeEos() {
        $r = SwitchManualIntSizeEos::fromFile(self::SRC_DIR_PATH . "/switch_tlv.bin");

        $this->assertEquals(4, count($r->chunks));

        $this->assertEquals(0x11, $r->chunks[0]->code);
        $meta = $r->chunks[0]->body->body;
        $this->assertEquals('Stuff', $meta->title);
        $this->assertEquals('Me', $meta->author);

        $this->assertEquals(0x22, $r->chunks[1]->code);
        $this->assertEquals(['AAAA', 'BBBB', 'CCCC'], $r->chunks[1]->body->body->entries);

        $this->assertEquals(0x33, $r->chunks[2]->code);
        $this->assertEquals("\x10\x20\x30\x40\x50\x60\x70\x80", $r->chunks[2]->body->body);

        $this->assertEquals(0xff, $r->chunks[3]->code);
        $this->assertEquals('', $r->chunks[3]->body->body);
    }
}
