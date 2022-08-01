package spec::perl::TestOpaqueExternalType02Parent;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use OpaqueExternalType02Parent;

sub test_opaque_external_type_02_parent: Test(3) {
    my $r = OpaqueExternalType02Parent->from_file('src/term_strz.bin');


    is($r->parent()->child()->s1(), "foo", 'Equals');
    is($r->parent()->child()->s2(), "bar", 'Equals');
    is($r->parent()->child()->s3()->s3(), "|baz\@", 'Equals');
}

Test::Class->runtests;
