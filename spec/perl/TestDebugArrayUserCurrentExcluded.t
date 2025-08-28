package spec::perl::TestDebugArrayUserCurrentExcluded;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use DebugArrayUserCurrentExcluded;

sub test_debug_array_user_current_excluded: Test(3) {
    my $r = DebugArrayUserCurrentExcluded->from_file('src/term_strz.bin');

    # --debug implies --no-auto-read
    $r->_read();

    is(@{$r->array_of_cats()}[0]->meow(), pack('C*', (102, 111, 111)), 'Equals');
    is(@{$r->array_of_cats()}[1]->meow(), pack('C*', (124, 98)), 'Equals');
    is(@{$r->array_of_cats()}[2]->meow(), pack('C*', (97)), 'Equals');
}

Test::Class->runtests;
