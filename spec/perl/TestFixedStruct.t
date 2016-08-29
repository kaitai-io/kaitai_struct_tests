package spec::perl::TestFixedStruct;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use FixedStruct;

sub test_fixed_struct: Test(20) {
    my $r = FixedStruct->from_file('src/fixed_struct.bin');

    is($r->hdr()->{uint8}, 255, 'Equals');
    is($r->hdr()->{uint16}, 65535, 'Equals');
    is($r->hdr()->{uint32}, 4294967295, 'Equals');
    is($r->hdr()->{uint64}, 18446744073709551615, 'Equals');

    is($r->hdr()->{sint8}, -1, 'Equals');
    is($r->hdr()->{sint16}, -1, 'Equals');
    is($r->hdr()->{sint32}, -1, 'Equals');
    is($r->hdr()->{sint64}, -1, 'Equals');

    is($r->hdr()->{uint16le}, 66, 'Equals');
    is($r->hdr()->{uint32le}, 66, 'Equals');
    is($r->hdr()->{uint64le}, 66, 'Equals');

    is($r->hdr()->{sint16le}, -66, 'Equals');
    is($r->hdr()->{sint32le}, -66, 'Equals');
    is($r->hdr()->{sint64le}, -66, 'Equals');

    is($r->hdr()->{uint16be}, 66, 'Equals');
    is($r->hdr()->{uint32be}, 66, 'Equals');
    is($r->hdr()->{uint64be}, 66, 'Equals');

    is($r->hdr()->{sint16be}, -66, 'Equals');
    is($r->hdr()->{sint32be}, -66, 'Equals');
    is($r->hdr()->{sint64be}, -66, 'Equals');
}

Test::Class->runtests;
