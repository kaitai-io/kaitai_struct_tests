# Autogenerated from KST: please remove this line if doing any edits by hand!

require 'cast_to_top'

RSpec.describe CastToTop do
  it 'parses test properly' do
    r = CastToTop.from_file('src/fixed_struct.bin')

    expect(r.code).to eq 80
    expect(r.header.code).to eq 65
    expect(r.header_casted.code).to eq 65
  end
end
