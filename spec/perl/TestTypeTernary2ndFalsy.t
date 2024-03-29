# Autogenerated from KST: please remove this line if doing any edits by hand!

package spec::perl::TestTypeTernary2ndFalsy;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use TypeTernary2ndFalsy;

sub test_type_ternary_2nd_falsy: Test(13) {
    my $r = TypeTernary2ndFalsy->from_file('src/switch_integers.bin');

    cmp_ok($r->v_false(), '==', 0, 'Equals');
    is($r->v_int_zero(), 0, 'Equals');
    is($r->v_int_neg_zero(), 0, 'Equals');
    ok(abs($r->v_float_zero() - 0.0) < 1e-6, 'Approx equals');
    ok(abs($r->v_float_neg_zero() - -0.0) < 1e-6, 'Approx equals');
    is($r->v_str_w_zero(), "0", 'Equals');
    is(length($r->v_str_w_zero()), 1, 'Equals');
    is($r->ut()->m(), 7, 'Equals');
    ok(!defined($r->v_null_ut()), 'nil');
    is($r->v_str_empty(), "", 'Equals');
    is(length($r->v_str_empty()), 0, 'Equals');
    is(scalar(@{$r->int_array()}), 2, 'Equals');
    is(scalar(@{$r->v_int_array_empty()}), 0, 'Equals');
}

Test::Class->runtests;
