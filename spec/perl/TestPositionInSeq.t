package spec::perl::TestPositionInSeq;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use PositionInSeq;

sub test_position_in_seq: Test(1) {
    my $r = PositionInSeq->from_file('src/position_in_seq.bin');

    is($r->numbers(), [1(), 2(), 3], 'Equals');
}

Test::Class->runtests;
