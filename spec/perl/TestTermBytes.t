package spec::perl::TestTermBytes;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use TermBytes;

sub test_term_bytes: Test(3) {
    my $r = TermBytes->from_file('src/term_strz.bin');

    is($r->s1(), 'foo', 'Equals');
    is($r->s2(), 'bar', 'Equals');
    is($r->s3(), '|baz@', 'Equals');
}

Test::Class->runtests;
