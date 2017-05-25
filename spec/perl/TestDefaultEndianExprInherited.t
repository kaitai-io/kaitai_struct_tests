package spec::perl::TestDefaultEndianExprInherited;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use DefaultEndianExprInherited;

sub test_default_endian_expr_inherited: Test(15) {
    my $r = DefaultEndianExprInherited->from_file('src/endian_expr.bin');

    is_deeply($r->docs()->[0]->indicator(), "II", 'Equals');
    is($r->docs()->[0]->main()->insides()->some_int(), 0x42, 'Equals');
    is($r->docs()->[0]->main()->insides()->more()->some_int1(), 0x4200, 'Equals');
    is($r->docs()->[0]->main()->insides()->more()->some_int2(), 0x42, 'Equals');
    is($r->docs()->[0]->main()->insides()->more()->some_inst(), 0x42, 'Equals');

    is_deeply($r->docs()->[1]->indicator(), "MM", 'Equals');
    is($r->docs()->[1]->main()->insides()->some_int(), 0x42, 'Equals');
    is($r->docs()->[1]->main()->insides()->more()->some_int1(), 0x42, 'Equals');
    is($r->docs()->[1]->main()->insides()->more()->some_int2(), 0x4200, 'Equals');
    is($r->docs()->[1]->main()->insides()->more()->some_inst(), 0x42000000, 'Equals');

    is_deeply($r->docs()->[2]->indicator(), "XX", 'Equals');
    is($r->docs()->[2]->main()->insides()->some_int(), 0x42, 'Equals');
    is($r->docs()->[2]->main()->insides()->more()->some_int1(), 0x42, 'Equals');
    is($r->docs()->[2]->main()->insides()->more()->some_int2(), 0x4200, 'Equals');
    is($r->docs()->[2]->main()->insides()->more()->some_inst(), 0x42000000, 'Equals');
}

Test::Class->runtests;
