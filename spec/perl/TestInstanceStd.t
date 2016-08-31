package spec::perl::TestInstanceStd;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use InstanceStd;

sub test_instance_std: Test {
    my $r = InstanceStd->from_file('src/str_encodings.bin');

    is($r->header(), 'Some ', 'Equals');
}

Test::Class->runtests;
