<?php
namespace Kaitai\Struct\Tests;

use Kaitai\Struct\Tests\HelloWorld;

class HelloWorldTest extends \PHPUnit_Framework_TestCase {
    const FILE_PATH = __DIR__ . '/../../src/fixed_struct.bin';
    public function testAccessProperty_ThroughDirectCall() {
        $r = HelloWorld::fromFile(self::FILE_PATH);
        $this->assertEquals(0x50, $r->one());
    }

    public function testAccessProperty_ThroughGetCall() {
        $r = HelloWorld::fromFile(self::FILE_PATH);
        $this->assertEquals(0x50, $r->one);
    }
}
