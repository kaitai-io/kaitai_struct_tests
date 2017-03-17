require 'nav_parent_false2'

RSpec.describe NavParentFalse2 do
  it 'parses test properly' do
    r = NavParentFalse2.from_file('src/fixed_struct.bin')

    expect(r.parentless.foo).to eq 80
  end
end
