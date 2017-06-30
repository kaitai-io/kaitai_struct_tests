package spec::perl::TestProcessCoerceBytes;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use ProcessCoerceBytes;

sub test_process_coerce_bytes: Test(4) {
    my $r = ProcessCoerceBytes->from_file('src/process_coerce_bytes.bin');

    is($r->records()->[0]->flag(), 0, 'Equals');
    is($r->records()->[0]->buf(), 'AAAA', 'Equals');
    is($r->records()->[1]->flag(), 1, 'Equals');
    is($r->records()->[1]->buf(), 'BBBB', 'Equals');
}

Test::Class->runtests;
