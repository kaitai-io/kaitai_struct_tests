package spec::perl::TestInstanceIoUser;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use InstanceIoUser;

sub test_instance_io_user: Test {
    my $r = InstanceIoUser->from_file("src/instance_io.bin");

    is($r->{qty_entries}, 3}, 'Equals');

    is($r->{entries[0]}->{name}, "the"}, 'Equals');
    is($r->{entries[1]}->{name}, "rainy"}, 'Equals');
    is($r->{entries[2]}->{name}, "day it is"}, 'Equals');
}

Test::Class->runtests;
