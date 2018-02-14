package spec::perl::TestDefaultEndianExprIsLe;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use DefaultEndianExprIsLe;

sub test_default_endian_expr_is_le: Test(12) {
    my $r = DefaultEndianExprIsLe->from_file('src/endian_expr.bin');

    is($r->docs()->[0]->indicator(), 'II', 'Equals');
    is($r->docs()->[0]->main()->some_int(), 0x42, 'Equals');
    is($r->docs()->[0]->main()->some_int_be(), 0x42, 'Equals');
    is($r->docs()->[0]->main()->some_int_le(), 0x42, 'Equals');

    is($r->docs()->[1]->indicator(), 'MM', 'Equals');
    is($r->docs()->[1]->main()->some_int(), 0x42, 'Equals');
    is($r->docs()->[1]->main()->some_int_be(), 0x42, 'Equals');
    is($r->docs()->[1]->main()->some_int_le(), 0x42, 'Equals');

    is($r->docs()->[2]->indicator(), 'XX', 'Equals');
    is($r->docs()->[2]->main()->some_int(), 0x42, 'Equals');
    is($r->docs()->[2]->main()->some_int_be(), 0x42, 'Equals');
    is($r->docs()->[2]->main()->some_int_le(), 0x42, 'Equals');
}

Test::Class->runtests;
