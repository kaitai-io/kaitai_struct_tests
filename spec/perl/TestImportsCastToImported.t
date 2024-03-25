# Autogenerated from KST: please remove this line if doing any edits by hand!

package spec::perl::TestImportsCastToImported;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use ImportsCastToImported;

sub test_imports_cast_to_imported: Test(2) {
    my $r = ImportsCastToImported->from_file('src/process_xor_4.bin');

    is($r->hw()->one(), 236, 'Equals');
    is($r->two()->hw_one(), 236, 'Equals');
}

Test::Class->runtests;