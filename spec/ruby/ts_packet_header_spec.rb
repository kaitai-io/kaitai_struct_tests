require 'ts_packet_header'

RSpec.describe TsPacketHeader do
  it 'parses test properly' do
    r = TsPacketHeader.from_file('src/ts_packet.bin')

    expect(r.sync_byte).to eq 0x47
    expect(r.transport_error_indicator).to eq false
    expect(r.payload_unit_start_indicator).to eq false
    expect(r.transport_priority).to eq true
    expect(r.pid).to eq 33
    expect(r.transport_scrambling_control).to eq 0
    expect(r.adaptation_field_control).to eq :adaptation_field_control_enum_payload_only
  end
end
