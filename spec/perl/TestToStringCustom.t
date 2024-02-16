package spec::perl::TestToStringCustom;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use ToStringCustom;

sub test_to_string_custom: Test(1) {
    my $r = ToStringCustom->from_file('src/term_strz.bin');

    is("$r", "s1 = foo, s2 = bar", 'Equals');
}

Test::Class->runtests;
