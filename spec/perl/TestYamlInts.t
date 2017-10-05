package spec::perl::TestYamlInts;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use YamlInts;

sub test_yaml_ints: Test(4) {
    my $r = YamlInts->from_file('src/fixed_struct.bin');

    is($r->test_u4_dec(), 0xffffffff, 'Equals');
    is($r->test_u4_hex(), 0xffffffff, 'Equals');
    is($r->test_u8_dec(), 0xffffffffffffffff, 'Equals');
    is($r->test_u8_hex(), 0xffffffffffffffff, 'Equals');
}

Test::Class->runtests;
