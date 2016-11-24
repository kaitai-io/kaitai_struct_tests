package spec::perl::TestStrEncodingsDefault;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use StrEncodingsDefault;
use utf8;

sub test_str_encodings_default: Test(4) {
    my $r = StrEncodingsDefault->from_file('src/str_encodings.bin');

    is($r->str1(), 'Some ASCII', 'Equals');
    is($r->rest()->str2(), 'こんにちは', 'Equals');
    is($r->rest()->str3(), 'こんにちは', 'Equals');
    is($r->rest()->str4(), '░▒▓', 'Equals');
}

Test::Class->runtests;
