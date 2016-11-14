package spec::perl::TestNestedSameName;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use NestedSameName;

sub test_nested_same_name: Test(2) {
    my $r = NestedSameName->from_file('src/repeat_n_struct.bin');

    is($r->main_data()->main_size(), 2, 'Equals');
    is($r->main_data()->foo()->data(), "\x10\0\0\0", 'Equals');
}

Test::Class->runtests;
