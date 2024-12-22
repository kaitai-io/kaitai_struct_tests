package spec::perl::TestDebugArrayUser;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use DebugArrayUser;

sub test_debug_array_user: Test(5) {
    my $r = DebugArrayUser->from_file('src/fixed_struct.bin');

    # --debug implies --no-auto-read
    $r->_read();

    is($r->one_cat()->meow(), 80, 'Equals');
    is(scalar(@{$r->array_of_cats()}), 3, 'Equals');
    is(@{$r->array_of_cats()}[0]->meow(), 65, 'Equals');
    is(@{$r->array_of_cats()}[1]->meow(), 67, 'Equals');
    is(@{$r->array_of_cats()}[2]->meow(), 75, 'Equals');
}

Test::Class->runtests;
