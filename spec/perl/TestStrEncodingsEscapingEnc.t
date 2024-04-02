package spec::perl::TestStrEncodingsEscapingEnc;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use StrEncodingsEscapingEnc;
use Test::Exception;

sub test_str_encodings_escaping_enc: Test(4) {
    my $r = StrEncodingsEscapingEnc->from_file('src/str_encodings.bin');

    throws_ok { $r->str1()->v() } qr/^Unknown encoding 'ASCII\\\\x'/;
    throws_ok { $r->str2()->v() } qr/^Unknown encoding 'UTF-8\\'x'/;
    throws_ok { $r->str3()->v() } qr/^Unknown encoding 'SJIS\\\"x'/;
    throws_ok { $r->str4()->v() } qr/^Unknown encoding 'IBM437\\nx'/;
}

Test::Class->runtests;
