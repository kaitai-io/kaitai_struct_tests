package spec::perl::TestDefaultEndianExprException;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use DefaultEndianExprException;
use Test::Exception;

sub test_default_endian_expr_exception: Test(1) {
    dies_ok { DefaultEndianExprException->from_file('src/endian_expr.bin'); } 'Died';
}

Test::Class->runtests;
