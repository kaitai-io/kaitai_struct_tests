<?php

include '../runtime/php/lib/Kaitai/Struct.php';
include '../runtime/php/lib/Kaitai/Stream.php';

spl_autoload_register(function ($name) {
    $kt = "Kaitai\\Tests\\";

    if (substr($name, 0, strlen($kt)) == $kt) {
        $testName = substr($name, strlen($kt));
        include "compiled/php/$testName.php";
    }
#    echo "Want to load $name.\n";
#    throw new Exception("Unable to load $name.");
});
