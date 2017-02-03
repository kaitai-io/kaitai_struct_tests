package spec::perl::TestBitsEnum;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use BitsEnum;

sub test_bits_enum: Test(3) {
    my $r = BitsEnum->from_file('src/fixed_struct.bin');

    # 50 41 (4 + 8 + 1) = 0101|0000 0100|0|001
    is($r->one(), $BitsEnum::ANIMAL_PLATYPUS, 'Equals');
    is($r->two(), $BitsEnum::ANIMAL_HORSE, 'Equals');
    is($r->three(), $BitsEnum::ANIMAL_CAT, 'Equals');
}

Test::Class->runtests;
