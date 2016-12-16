require 'parent_vs_value_inst'

RSpec.describe ParentVsValueInst do
  it 'parses test properly' do
    r = ParentVsValueInst.from_file('src/term_strz.bin')

    expect(r.s1).to eq('foo')
  end
end
