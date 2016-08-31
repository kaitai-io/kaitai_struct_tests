package spec::perl::TestProcessXorConst;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use ProcessXorConst;

sub test_process_xor_const: Test {
    my $r = ProcessXorConst->from_file('src/process_xor_1.bin');

    is($r->{key}, 0xff, 'Equals');
    is($r->{buf}, 'foo bar', 'Equals');
}

Test::Class->runtests;
