package spec::perl::TestExprIoPos;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use ExprIoPos;

sub test_expr_io_pos: Test(6) {
    my $r = ExprIoPos->from_file('src/expr_io_pos.bin');

    is($r->substream1()->my_str(), 'CURIOSITY', 'Equals');
    is_deeply($r->substream1()->body(), pack('C*', (0x11, 0x22, 0x33, 0x44)), 'Equals');
    is($r->substream1()->number(), 0x42, 'Equals');

    is($r->substream2()->my_str(), 'KILLED', 'Equals');
    is($r->substream2()->body(), "a cat", 'Equals');
    is($r->substream2()->number(), 0x67, 'Equals');
}

Test::Class->runtests;
