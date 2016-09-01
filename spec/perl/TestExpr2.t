package spec::perl::TestExpr2;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
#use Expr2;
#
#sub test_expr_2: Test(16) {
#    my $r = Expr2->from_file('src/str_encodings.bin');
#
#    is($r->{str1}->{len_orig}, 10, 'Equals');
#    is($r->{str1}->len_mod(), 7, 'Equals');
#    is($r->{str1}->{str}, "Some AS", 'Equals');
#
#    is($r->str1_len(), 7, 'Equals');
#    is($r->str1_len_mod(), 7, 'Equals');
#    is($r->str1_byte1(), 0x49, 'Equals');
#    is($r->str1_avg(), 0x49, 'Equals');
#    is($r->str1_char5(), 'e', 'Equals');
#
#    is($r->str1_tuple5()->{byte0}, 0x65, 'Equals');
#    is($r->str1_tuple5()->{byte1}, 0x20, 'Equals');
#    is($r->str1_tuple5()->{byte2}, 0x41, 'Equals');
#    is($r->str1_tuple5()->avg(), 0x30, 'Equals');
#
#    is($r->str2_tuple5()->{byte0}, 0x65, 'Equals');
#    is($r->str2_tuple5()->{byte1}, 0x20, 'Equals');
#    is($r->str2_tuple5()->{byte2}, 0x41, 'Equals');
#    is($r->str2_tuple5()->avg(), 0x30, 'Equals');
#}
#
#Test::Class->runtests;
