package spec::perl::TestUserType;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use UserType;

sub test_user_type: Test(2) {
    my $r = UserType->from_file('src/repeat_until_s4.bin');

    is($r->one()->width(), 0x42, 'Equals');
    is($r->one()->height(), 0x1337, 'Equals');
}

Test::Class->runtests;
