require_relative '../../java_builder'

$spec_dir = File.dirname(__FILE__)

RSpec.describe JavaBuilder do
  before :context do
    @spec_dir = File.dirname(__FILE__)
  end

  context '1' do
    before :context do
      Dir.chdir("#{@spec_dir}/1")
      @builder = JavaBuilder.new('spec')
    end

    describe '#parse_failed_build' do
      it 'parses failed build information' do
        expect(@builder.parse_failed_build('test_out/java/build-1.log').to_a.sort).to eq(
          ['compiled/java/src/io/kaitai/struct/testformats/ExprMod.java']
        )
      end
    end

    describe '#file_to_test' do
      it 'parses spec filename' do
        expect(
          @builder.file_to_test('spec/java/src/io/kaitai/struct/spec/TestCastNested.java')
        ).to eq [:spec, 'CastNested']
      end

      it 'parses format filename' do
        expect(
          @builder.file_to_test('compiled/java/src/io/kaitai/struct/testformats/ExprMod.java')
        ).to eq [:format, 'ExprMod']
      end
    end
  end
end
