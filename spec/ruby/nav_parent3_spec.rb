require 'nav_parent3'

RSpec.describe NavParent3 do
  it 'parses test properly' do
    r = NavParent3.from_file('src/nav_parent2.bin')

    expect(r.ofs_tags).to eq 8
    expect(r.num_tags).to eq 2

    expect(r.tags[0].name).to eq 'RAHC'
    expect(r.tags[0].ofs).to eq 0x20
    expect(r.tags[0].num_items).to eq 3
    expect(r.tags[0].tag_content.content).to eq 'foo'

    expect(r.tags[1].name).to eq 'RAHC'
    expect(r.tags[1].ofs).to eq 0x23
    expect(r.tags[1].num_items).to eq 6
    expect(r.tags[1].tag_content.content).to eq 'barbaz'
  end
end
