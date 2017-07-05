require 'meta_xref'

RSpec.describe MetaXref do
  it 'parses test properly' do
    r = MetaXref.from_file('src/fixed_struct.bin')
  end
end
