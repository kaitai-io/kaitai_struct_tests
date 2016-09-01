package spec::perl::TestInstanceUserArray;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
#use InstanceUserArray;
#
#sub test_instance_user_array: Test {
#    my $r = InstanceUserArray->from_file("src/instance_std_array.bin");
#
#    is($r->{ofs}, 0x10, 'Equals');
#    is($r->{qty_entries}, 3}, 'Equals');
#    is($r->{entry_size}, 4}, 'Equals');
#
#    is($r->{user_entries[0]}->{word1}, 0x1111, 'Equals');
#    is($r->{user_entries[0]}->{word2}, 0x1111, 'Equals');
#    is($r->{user_entries[1]}->{word1}, 0x2222, 'Equals');
#    is($r->{user_entries[1]}->{word2}, 0x2222, 'Equals');
#    is($r->{user_entries[2]}->{word1}, 0x3333, 'Equals');
#    is($r->{user_entries[2]}->{word2}, 0x3333, 'Equals');
#}
#
#Test::Class->runtests;
