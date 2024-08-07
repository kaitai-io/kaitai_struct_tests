# Autogenerated from KST: please remove this line if doing any edits by hand!

package spec::perl::TestTermStrzUtf16V3;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use TermStrzUtf16V3;

sub test_term_strz_utf16_v3: Test(4) {
    my $r = TermStrzUtf16V3->from_file('src/term_strz_utf16.bin');

    is($r->s1(), "a\N{U+0200}b", 'Equals');
    is($r->term(), 0, 'Equals');
    is($r->s2(), "c\N{U+0200}d", 'Equals');
    is($r->s3(), "", 'Equals');
}

Test::Class->runtests;
