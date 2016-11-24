package spec::perl::TestDefaultEndianMod;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use DefaultEndianMod;

sub test_default_endian_mod: Test(3) {
    my $r = DefaultEndianMod->from_file('src/fixed_struct.bin');

    is($r->main()->one(), 0x4b434150, 'Equals');
    is($r->main()->nest()->two(), -52947, 'Equals');
    is($r->main()->nest_be()->two(), 0x5041434b, 'Equals');
}

Test::Class->runtests;
