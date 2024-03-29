# Autogenerated from KST: please remove this line if doing any edits by hand!

package spec::perl::TestRepeatNStrz;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use RepeatNStrz;

sub test_repeat_n_strz: Test(2) {
    my $r = RepeatNStrz->from_file('src/repeat_n_strz.bin');

    is($r->qty(), 2, 'Equals');
    is_deeply($r->lines(), ["foo", "bar"], 'Equals');
}

Test::Class->runtests;
