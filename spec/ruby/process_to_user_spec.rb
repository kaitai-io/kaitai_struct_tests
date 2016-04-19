require 'process_to_user'

RSpec.describe ProcessToUser do
  it 'parses test properly' do
    r = ProcessToUser.from_file('src/process_rotate.bin')

    expect(r.buf1.str).to eq 'Hello'
  end
end
