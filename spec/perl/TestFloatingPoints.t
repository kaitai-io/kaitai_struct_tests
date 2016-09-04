package spec::perl::TestFloatingPoints;

use strict;
use warnings;
use base qw(Test::Class);
use Test::More;
use FloatingPoints;

sub test_floating_points: Test(8) {
    my $r = FloatingPoints->from_file('src/floating_points.bin');

    is($r->single_value(), 0.5, 'Equals');
    is($r->single_value_be(), 0.5, 'Equals');
    is($r->double_value(), 0.25, 'Equals');
    is($r->double_value_be(), 0.25, 'Equals');

    my $val_1 = sprintf("%.4f", $r->approximate_value());
    is($val_1, 1.2345, 'Equals');

    my $val_2 = sprintf("%.1f", $r->single_value_plus_int());
    is($val_2, 1.5, 'Equals');

    my $val_3 = sprintf("%.1f", $r->single_value_plus_float());
    my $val_4 = sprintf("%.1f", 1.0);
    is($val_3, $val_4, 'Equals');

    my $val_5 = sprintf("%.1f", $r->double_value_plus_float());
    is($val_5, 0.3, 'Equals');
}

Test::Class->runtests;
