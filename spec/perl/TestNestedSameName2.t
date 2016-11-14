package spec::perl::TestNestedSameName2;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use NestedSameName2;

sub test_nested_same_name2: Test(5) {
    my $r = NestedSameName2->from_file('src/nested_same_name2.bin');

    is($r->version(), 0x42, 'Equals');
    is($r->main_data()->main_size(), 2, 'Equals');
    is($r->main_data()->foo()->data1(), "\x11\x11\x11\x11", 'Equals');
    is($r->dummy()->dummy_size(), 3, 'Equals');
    is($r->dummy()->foo()->data2(), "\x22\x22\x22\x22\x22\x22", 'Equals');
}

Test::Class->runtests;
