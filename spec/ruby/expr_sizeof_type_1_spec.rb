# Autogenerated from KST: please remove this line if doing any edits by hand!

RSpec.describe 'ExprSizeofType1' do
  it 'parses test properly' do
    require 'expr_sizeof_type_1'
    r = ExprSizeofType1.from_file('src/fixed_struct.bin')

    expect(r.sizeof_block).to eq (((1 + 4) + 2) + 4)
    expect(r.sizeof_subblock).to eq 4
  end
end
