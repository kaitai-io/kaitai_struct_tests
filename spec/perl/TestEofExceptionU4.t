package spec::perl::TestEofExceptionU4;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use EofExceptionU4;
use Test::Exception;

sub test_eof_exception_u4: Test(1) {
    dies_ok { EofExceptionU4->from_file('src/term_strz.bin') } 'Died';
}

Test::Class->runtests;
