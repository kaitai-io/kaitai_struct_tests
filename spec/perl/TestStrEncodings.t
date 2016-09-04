package spec::perl::TestStrEncodings;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use StrEncodings;
use utf8;

sub test_str_encodings: Test(4) {
    my $r = StrEncodings->from_file('src/str_encodings.bin');

    is($r->str1(), 'Some ASCII', 'Equals');
    is($r->str2(), 'こんにちは', 'Equals');
    is($r->str3(), 'こんにちは', 'Equals');
    is($r->str4(), '░▒▓', 'Equals');
}

Test::Class->runtests;
