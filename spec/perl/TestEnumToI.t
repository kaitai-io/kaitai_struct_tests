package spec::perl::TestEnumToI;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use EnumToI;

sub test_enum_to_i: Test(4) {
    my $r = EnumToI->from_file('src/enum_0.bin');

    is($r->pet_1(), $EnumToI::ANIMAL_CAT, 'Equals');
    is($r->pet_2(), $EnumToI::ANIMAL_CHICKEN, 'Equals');

    is($r->pet_1_i(), 7, 'Equals');
    is($r->one_lt_two(), 1, 'Equals');
}

Test::Class->runtests;
