require 'nav_parent_switch'

RSpec.describe NavParentSwitch do
  it 'parses test properly' do
    r = NavParentSwitch.from_file('src/nav_parent_switch.bin')

    expect(r.category).to eq 1
    expect(r.content.foo).to eq 0x42
    expect(r.content.subelement.bar).to eq 0xff
  end
end
