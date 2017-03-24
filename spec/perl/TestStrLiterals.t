package spec::perl::TestStrLiterals;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use StrLiterals;

sub test_str_literals: Test(5) {
    my $r = StrLiterals->from_file('src/fixed_struct.bin');

    is($r->complex_str(), "\x{0}\x{1}\x{2}\x{7}\x{8}\x{a}\x{d}\x{9}\x{b}\x{c}\x{1b}\x{3d}\x{7}\x{a}\x{24}\x{263b}", 'Equals');
    is($r->double_quotes(), "\x22\x22\x22", 'Equals');
    is($r->backslashes(), "\x5c\x5c\x5c", 'Equals');
    is($r->octal_eatup(), "\x00\x32\x32", 'Equals');
    is($r->octal_eatup2(), "\x02\x32", 'Equals');
}

Test::Class->runtests;
