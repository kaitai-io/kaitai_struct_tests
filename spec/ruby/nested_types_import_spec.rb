# Autogenerated from KST: please remove this line if doing any edits by hand!

RSpec.describe 'NestedTypesImport' do
  it 'parses test properly' do
    require 'nested_types_import'
    r = NestedTypesImport.from_file('src/fixed_struct.bin')

    expect(r.a_cc.value_cc).to eq 80
    expect(r.a_c_d.value_d).to eq 65
    expect(r.b.value_b).to eq 67
    expect(r.b.a_cc.value_cc).to eq 75
    expect(r.b.a_c_d.value_d).to eq 45
    expect(r.a_cc._parent).to be_nil
    expect(r.a_cc._root).to be_nil
    expect(r.a_c_d._parent).to be_nil
    expect(r.a_c_d._root).to be_nil
    expect(r.b._parent).to be_nil
    expect(r.b._root).to be_nil
  end
end
