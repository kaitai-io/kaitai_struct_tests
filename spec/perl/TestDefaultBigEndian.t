package spec::perl::TestDefaultBigEndian;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use DefaultBigEndian;

sub test_default_big_endian: Test(1) {
    my $r = DefaultBigEndian->from_file('src/enum_0.bin');

    is($r->{one}, 0x7000000, 'Equals');
}

Test::Class->runtests;
