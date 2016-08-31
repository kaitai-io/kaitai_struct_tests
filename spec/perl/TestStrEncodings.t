package spec::perl::TestStrEncodings;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use StrEncodings;

sub test_str_encodings: Test {
# coding: utf-8    my $r = StrEncodings->from_file("src/str_encodings.bin");

    is($r->{str1}, "Some ASCII"}, 'Equals');
    is($r->{str2}, u"こんにちは"}, 'Equals');
    is($r->{str3}, u"こんにちは"}, 'Equals');
    is($r->{str4}, u"░▒▓"}, 'Equals');
}

Test::Class->runtests;
