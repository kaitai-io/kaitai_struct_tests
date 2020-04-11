require 'imports_abs_abs'
require 'imported_and_abs'
require 'imported_root'

RSpec.describe ImportsAbsAbs do
  it 'parses test properly' do
    r = ImportsAbsAbs.from_file('src/fixed_struct.bin')

    expect(r.one).to eq 80
    expect(r.two.one).to eq 65
    expect(r.two.two.one).to eq 67
  end
end
