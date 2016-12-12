package spec::perl::TestTypeTernary;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use TypeTernary;

sub test_type_ternary: Test(1) {
    my $r = TypeTernary->from_file('src/term_strz.bin');

    is($r->dif()->value(), 0x65, 'Equals');
}

Test::Class->runtests;
