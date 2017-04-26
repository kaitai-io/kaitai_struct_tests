package spec::perl::TestExprBytesCmp;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use ExprBytesCmp;

sub test_expr_bytes_cmp: Test(10) {
    my $r = ExprBytesCmp->from_file('src/fixed_struct.bin');

    is($r->one(), 'P', 'Equals');
    is($r->two(), 'ACK', 'Equals');

    is($r->is_eq(), 1, 'Equals');
    is($r->is_ne(), '', 'Equals');
    is($r->is_lt(), 1, 'Equals');
    is($r->is_gt(), '', 'Equals');
    is($r->is_le(), 1, 'Equals');
    is($r->is_ge(), '', 'Equals');
    is($r->is_lt2(), '', 'Equals');
    is($r->is_gt2(), 1, 'Equals');
}

Test::Class->runtests;
