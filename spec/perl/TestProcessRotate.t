package spec::perl::TestProcessRotate;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use ProcessRotate;

sub test_process_rotate: Test {
    my $r = ProcessRotate->from_file("src/process_rotate.bin");

    is($r->{buf1}, "Hello"}, 'Equals');
    is($r->{buf2}, "World"}, 'Equals');
    is($r->{buf3}, "There"}, 'Equals');
}

Test::Class->runtests;
