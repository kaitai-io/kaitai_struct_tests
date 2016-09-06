package spec::perl::TestExpr3;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use Expr3;

sub test_expr_3: Test(16) {
    my $r = Expr3->from_file('src/fixed_struct.bin');

    is($r->one, 80, 'Equals');
    is($r->two, 'ACK', 'Equals');

    is($r->three, '@ACK', 'Equals');
    is($r->four, '_ACK_', 'Equals');
    ok($r->is_str_eq);
    ok(!$r->is_str_ne);
    ok($r->is_str_lt);
    ok(!$r->is_str_gt);
    ok($r->is_str_le);
    ok(!$r->is_str_ge);
    ok($r->is_str_lt2);
}

Test::Class->runtests;
