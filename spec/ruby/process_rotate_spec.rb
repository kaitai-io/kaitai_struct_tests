require 'process_rotate'

RSpec.describe ProcessRotate do
  it 'parses test properly' do
    r = ProcessRotate.from_file('src/process_rotate.bin')

    expect(r.buf1).to eq 'Hello'
    expect(r.buf2).to eq 'World'
    expect(r.buf3).to eq 'There'
  end
end
