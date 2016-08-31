package spec::perl::TestProcessXorValue;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use ProcessXorValue;

sub test_process_xor_value : Test(2) {
    my $r = ProcessXorValue->from_file('src/process_xor_1.bin');

    is($r->{key}, 0xff, 'equals');
    is($r->{buf}, "foo bar", 'equals');
}

Test::Class->runtests;
