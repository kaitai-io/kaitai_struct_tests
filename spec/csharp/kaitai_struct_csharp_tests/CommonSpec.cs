using System;
using System.IO;
using System.Text;

namespace Kaitai
{
    public abstract class CommonSpec
    {
#if NET6_0_OR_GREATER
        static CommonSpec()
        {
            // Use the `System.Text.Encoding.CodePages` package, which is required in .NET 6.0+ for
            // almost all but a few basic encodings to work - see
            // https://learn.microsoft.com/en-us/dotnet/api/system.text.encodingprovider?view=net-8.0#remarks
            Encoding.RegisterProvider(CodePagesEncodingProvider.Instance);
        }
#endif

        private string _testPrefix;

        /// <summary>
        /// Checks if a proposed directory legitimately hosts our binary input files.
        /// </summary>
        /// <param name="prefix"></param>
        /// <returns>True if we have hit the directory with our binary input files.</returns>
        /// <remarks>We test if "fixed_struct.bin" exists in a given prefix.</remarks>
        private bool CheckSrcPathPrefix(string prefix)
        {
            if (string.IsNullOrEmpty(prefix))
            {
                return false;
            }

            string fileToCheck = Path.Combine(prefix, "fixed_struct.bin");
            return File.Exists(fileToCheck);
        }

        /// <summary>
        /// Detects where the input binary files are located. Tests several possible locations,
        /// trying to be flexible about the location of the test suite.
        /// </summary>
        /// <returns>Path to the directory with input binary files.</returns>
        private string DetectTestPrefix()
        {
            // If already decided and cached, return previous decision
            if (_testPrefix != null)
            {
                return _testPrefix;
            }

            // Try #1: test explicit override from env variable
            _testPrefix = Environment.GetEnvironmentVariable("CSHARP_TEST_SRC_PATH");
            if (CheckSrcPathPrefix(_testPrefix))
            {
                return _testPrefix;
            }

            // Try #2: test if we're in the project output directory, e.g.:
            // "/tests/spec/csharp/kaitai_struct_csharp_tests_netstd/bin/Debug/net6.0"
            // and the binary input was properly copied to become "src" of it:
            _testPrefix = "src";
            if (CheckSrcPathPrefix(_testPrefix))
            {
                return _testPrefix;
            }

            // Try #3: test if we're in the project source directory, e.g.:
            // "/tests/spec/csharp/kaitai_struct_csharp_tests_netstd"
            // and the binary input is three levels up + "src":
            // "/tests/src"
            _testPrefix = Path.Combine("..", "..", "..", "src");
            if (CheckSrcPathPrefix(_testPrefix))
            {
                return _testPrefix;
            }

            throw new Exception("Unable to find binary input files anywhere, all tests will fail");
        }

        /// <summary>
        /// Generates full path to input binary file to parse. Detects directory with such files, if necessary.
        /// </summary>
        /// <param name="filename">.bin file to search for</param>
        /// <returns>full path to designated input binary file</returns>
        public string SourceFile(string filename)
        {
            return Path.Combine(DetectTestPrefix(), filename);
        }
    }
}
