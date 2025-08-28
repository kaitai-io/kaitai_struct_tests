package spec::perl::TestDebugArrayUserEofException;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use DebugArrayUserEofException;
use Test::Exception;

sub test_debug_array_user_eof_exception: Test(9) {
    my $r = DebugArrayUserEofException->from_file('src/nav_parent_codes.bin');

    # --debug implies --no-auto-read
    throws_ok { $r->_read() } '/^Requested \d+ bytes, but only \d+ bytes available/';

    is($r->one_cat()->meow(), 3, 'Equals');
    is($r->one_cat()->chirp(), 73, 'Equals');
    is(scalar(@{$r->array_of_cats()}), 3, 'Equals');
    is(@{$r->array_of_cats()}[0]->meow(), 49, 'Equals');
    is(@{$r->array_of_cats()}[0]->chirp(), 50, 'Equals');
    is(@{$r->array_of_cats()}[1]->meow(), 51, 'Equals');
    is(@{$r->array_of_cats()}[1]->chirp(), 66, 'Equals');
    is(@{$r->array_of_cats()}[2]->meow(), 98, 'Equals');
}

Test::Class->runtests;
