package spec::perl::TestInstanceStdArray;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
#use InstanceStdArray;
#
#sub test_instance_std_array: Test {
#    my $r = InstanceStdArray->from_file("src/instance_std_array.bin");
#
#    is($r->{ofs}, 0x10, 'Equals');
#    is($r->{qty_entries}, 3}, 'Equals');
#    is($r->{entry_size}, 4}, 'Equals');
#
#    is($r->{entries}, [;
#            "\x11\x11\x11\x11",
#            "\x22\x22\x22\x22",
#            "\x33\x33\x33\x33",
#        ])
#}
#
#Test::Class->runtests;
