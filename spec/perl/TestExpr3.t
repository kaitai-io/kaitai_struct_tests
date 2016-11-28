package spec::perl::TestExpr3;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use Expr3;

sub test_expr_3: Test(12) {
    my $r = Expr3->from_file('src/fixed_struct.bin');

    is($r->one(), 80, 'Equals');
    is($r->two(), 'ACK', 'Equals');

    is($r->three(), '@ACK', 'Equals');
    is($r->four(), '_ACK_', 'Equals');
    is($r->is_str_eq(), 1, 'Equals');
    is($r->is_str_ne(), '', 'Equals');
    is($r->is_str_lt(), 1, 'Equals');
    is($r->is_str_gt(), '', 'Equals');
    is($r->is_str_le(), 1, 'Equals');
    is($r->is_str_ge(), '', 'Equals');
    is($r->is_str_lt2(), 1, 'Equals');
    is($r->test_not(), 1, 'Equals');
}

Test::Class->runtests;
