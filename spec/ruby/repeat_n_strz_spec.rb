require 'repeat_n_strz'

RSpec.describe Repeat_n_strz do
  it 'parses test properly' do
    r = Repeat_n_strz.from_file('src/repeat_n_strz.bin')

    expect(r.qty).to eq 2
    expect(r.lines).to eq ['foo', 'bar']
  end
end
