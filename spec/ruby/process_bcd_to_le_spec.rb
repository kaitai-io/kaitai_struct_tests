require 'process_bcd_to_le'

RSpec.describe ProcessBcdToLe do
  it 'parses test properly' do
    r = ProcessBcdToLe.from_file('src/process_bcd_to_le.bin')

    expect(r.ltr.value).to eq "12345678"
    expect(r.rtl.value).to eq "78563412"
    expect(r.decimal.value).to eq 12345678
  end
end
