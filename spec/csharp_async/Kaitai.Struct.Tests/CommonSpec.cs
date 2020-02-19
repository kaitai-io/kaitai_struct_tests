using System;
using System.IO;
using NUnit.Framework;

namespace Kaitai
{
    public class CommonSpec
    {
        public String SourceFile(string filename)
        {
            string testPath = Environment.GetEnvironmentVariable("CSHARP_TEST_SRC_PATH");
            if (testPath == null)
            {
                testPath = Path.Combine(TestContext.CurrentContext.TestDirectory, "src");
            }
            return Path.Combine(testPath, filename);
        }
    }
}
