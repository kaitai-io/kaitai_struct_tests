package spec::perl::TestTypeTernaryOpaque;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use TypeTernaryOpaque;

sub test_type_ternary_opaque: Test(3) {
    my $r = TypeTernaryOpaque->from_file('src/term_strz.bin');

    is($r->dif()->s1(), 'foo', 'Equals');
    is($r->dif()->s2(), 'bar', 'Equals');
    is($r->dif()->s3(), '|baz@', 'Equals');
}

Test::Class->runtests;
