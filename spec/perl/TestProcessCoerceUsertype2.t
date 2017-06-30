package spec::perl::TestProcessCoerceUsertype2;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use ProcessCoerceUsertype2;

sub test_process_coerce_usertype2: Test(4) {
    my $r = ProcessCoerceUsertype2->from_file('src/process_coerce_bytes.bin');

    is($r->records()->[0]->flag(), 0, 'Equals');
    is($r->records()->[0]->buf()->value(), 0x41414141, 'Equals');
    is($r->records()->[1]->flag(), 1, 'Equals');
    is($r->records()->[1]->buf()->value(), 0x42424242, 'Equals');
}

Test::Class->runtests;
