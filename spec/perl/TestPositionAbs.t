package spec::perl::TestPositionAbs;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use PositionAbs;

sub test_position_abs: Test(2) {
    my $r = PositionAbs->from_file("src/position_abs.bin");

    is($r->{index_offset}, 0x20, 'Equals');
    is($r->index()->{entry}, 'foo', 'Equals');
}

Test::Class->runtests;
