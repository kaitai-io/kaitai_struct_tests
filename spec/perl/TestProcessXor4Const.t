NUMBERpackage spec::perl::TestProcessXor4Const;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use ProcessXor4Const;

sub test_process_xor4_const: Test {
    my $r = ProcessXor4Const->from_file("src/process_xor_4.bin");

    is($r->{key}, bytearray([0xec}, 0xbb}, 0xa3}, 0x14], 'Equals'));
    is($r->{buf}, "foo bar"}, 'Equals');
}

Test::Class->runtests;
