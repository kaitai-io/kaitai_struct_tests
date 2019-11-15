<?php
namespace Kaitai\Struct\Tests;

class MyCustomFx {
    public function __construct($key, $flag, $someBytes) {
        $this->key = $flag ? $key : -$key;
    }

    public function decode($src) {
        $dst = '';
        for ($i = 0, $n = strlen($src); $i < $n; $i++) {
            $dst .= chr(ord($src[$i]) + $this->key);
        }
        return $dst;
    }
}

namespace Nested\Deeply;

class CustomFx {
    public function decode($src) {
        return "_" . $src . "_";
    }
}
