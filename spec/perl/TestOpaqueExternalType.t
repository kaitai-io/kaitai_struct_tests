package spec::perl::TestOpaqueExternalType;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use OpaqueExternalType;

sub test_opaque_external_type: Test(3) {
    my $r = OpaqueExternalType->from_file('src/term_strz.bin');

    is($r->one()->s1(), 'foo', 'Equals');
    is($r->one()->s2(), 'bar', 'Equals');
    is($r->one()->s3(), '|baz@', 'Equals');
}

Test::Class->runtests;
