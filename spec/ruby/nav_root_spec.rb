require 'nav_root'

RSpec.describe NavRoot do
  it 'parses test properly' do
    r = NavRoot.from_file('src/nav.bin')

    expect(r.header.qtyEntries).to eq 2
    expect(r.header.filenameLen).to eq 8

    expect(r.index.entries.size).to eq 2
    expect(r.index.entries[0].filename).to eq 'FIRST___'
    expect(r.index.entries[1].filename).to eq 'SECOND__'
  end
end
