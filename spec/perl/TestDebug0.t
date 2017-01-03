package spec::perl::TestDebug0;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use Debug0;

sub test_debug_0: Test(0) {
    # no debug mode so far, nothing to test
}

Test::Class->runtests;
