# Autogenerated from KST: please remove this line if doing any edits by hand!

require 'nested_type_param'

RSpec.describe NestedTypeParam do
  it 'parses test properly' do
    r = NestedTypeParam.from_file('src/term_strz.bin')

    expect(r.main_seq.my_len).to eq 5
    expect(r.main_seq.body).to eq "foo|b"
  end
end
