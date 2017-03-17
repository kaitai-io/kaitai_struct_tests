require 'nav_parent_false'

RSpec.describe NavParentFalse do
  it 'parses test properly' do
    r = NavParentFalse.from_file('src/nav_parent_codes.bin')

    expect(r.child_size).to eq 3
    expect(r.element_a.foo.code).to eq 73
    expect(r.element_a.foo.more).to eq [49, 50, 51].pack('C*')
    expect(r.element_a.bar.foo.code).to eq 66
    expect(r.element_b.foo.code).to eq 98
  end
end
