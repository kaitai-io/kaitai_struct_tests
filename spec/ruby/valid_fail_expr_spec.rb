# Autogenerated from KST: please remove this line if doing any edits by hand!

RSpec.describe 'ValidFailExpr' do
  it 'parses test properly' do
    require 'valid_fail_expr'
    expect {
      r = ValidFailExpr.from_file('src/nav_parent_switch.bin')
    }.to raise_error(Kaitai::Struct::ValidationExprError)
  end
end
