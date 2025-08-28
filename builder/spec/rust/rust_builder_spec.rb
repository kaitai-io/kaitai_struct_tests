require_relative '../../rust_builder'

# From https://stackoverflow.com/a/64278755
RSpec.configure do |rspec|
  rspec.expect_with :rspec do |c|
    c.max_formatted_output_length = nil
  end
end

RSpec.describe RustBuilder do
  before :context do
    @spec_dir = File.dirname(__FILE__)
  end

  # This case tests that RustBuilder can correctly identify the bad test file if
  # it triggers a compile error via macro expansion. For example, the use of
  # `assert_eq!` with incompatible types in a test spec will be expanded to a
  # `==` operation in the `library/core/src/macros/mod.rs` file, which `rustc`
  # will report as the primary error location in the JSON output of the build.
  # However, we want the file name of the test spec that called the `assert_eq!`
  # macro so we can exclude it from future builds. Therefore, RustBuilder must
  # descend deeper into the JSON to get the location of the outermost macro
  # invocation, which should be in the offending test spec.
  #
  # To demonstrate the recursive nature of the problem,
  # `spec/rust/tests/test_nested.rs` doesn't use `assert_eq!` directly but via
  # another `our_assert_eq_wrapper!` macro (this is obviously contrived, but we
  # can't rule out that such scenario could happen in practice, so RustBuilder
  # should support it). This means that we apply the descend step
  # `span["expansion"]["span"]` twice to get the test spec name, since the JSON
  # output looks like this (simplified):
  #
  # ```json
  #   "message": {
  #     "$message_type": "diagnostic",
  #     "level": "error",
  #     "message": "can't compare `{integer}` with `{float}`",
  #     "spans": [
  #       {
  #         "expansion": {
  #           "macro_decl_name": "assert_eq!",
  #           "span": {
  #             "expansion": {
  #               "macro_decl_name": "our_assert_eq_wrapper!",
  #               "span": {
  #                 "expansion": null,
  #                 "file_name": "tests/test_nested.rs",
  #                 "is_primary": false
  #               }
  #             },
  #             "file_name": "/mnt/c/temp/kaitai_struct/tests/builder/spec/rust/macro_expansion/spec/rust/src/helpers.rs",
  #             "is_primary": false
  #           }
  #         },
  #         "file_name": "/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/macros/mod.rs",
  #         "is_primary": true
  #       }
  #     ]
  #   }
  # ```
  context 'macro_expansion' do
    before :context do
      Dir.chdir("#{@spec_dir}/macro_expansion")
      @builder = RustBuilder.new
    end

    describe '#parse_failed_build' do
      it 'parses failed build information' do
        test_nested_path = File.absolute_path('spec/rust/tests/test_nested.rs')
        expect(@builder.parse_failed_build('test_out/rust/build-1.log')).to eq [
          test_nested_path,
          test_nested_path,
        ]
      end
    end
  end
end
