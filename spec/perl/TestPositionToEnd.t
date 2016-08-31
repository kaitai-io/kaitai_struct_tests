NUMBERNUMBERpackage spec::perl::TestPositionToEnd;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use PositionToEnd;

sub test_position_to_end: Test {
    my $r = PositionToEnd->from_file("src/position_to_end.bin");

    is($r->{index}->{foo}, 0x42, 'Equals');
    is($r->{index}->{bar}, 0x1234, 'Equals');
}

Test::Class->runtests;
