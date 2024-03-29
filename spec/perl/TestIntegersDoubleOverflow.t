# Autogenerated from KST: please remove this line if doing any edits by hand!

package spec::perl::TestIntegersDoubleOverflow;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use IntegersDoubleOverflow;

sub test_integers_double_overflow: Test(8) {
    my $r = IntegersDoubleOverflow->from_file('src/integers_double_overflow.bin');

    is($r->signed_safe_min_be(), -9007199254740991, 'Equals');
    is($r->signed_safe_min_le(), -9007199254740991, 'Equals');
    is($r->signed_safe_max_be(), 9007199254740991, 'Equals');
    is($r->signed_safe_max_le(), 9007199254740991, 'Equals');
    is(sprintf('%d', $r->signed_unsafe_neg_be()), "-9007199254740993", 'Equals');
    is(sprintf('%d', $r->signed_unsafe_neg_le()), "-9007199254740993", 'Equals');
    is(sprintf('%d', $r->signed_unsafe_pos_be()), "9007199254740993", 'Equals');
    is(sprintf('%d', $r->signed_unsafe_pos_be()), "9007199254740993", 'Equals');
}

Test::Class->runtests;
