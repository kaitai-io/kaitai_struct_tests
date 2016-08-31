package spec::perl::TestProcessToUser;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use ProcessToUser;

sub test_process_to_user: Test {
    my $r = ProcessToUser->from_file("src/process_rotate.bin");

    is($r->{buf1}->{str}, "Hello"}, 'Equals');
}

Test::Class->runtests;
