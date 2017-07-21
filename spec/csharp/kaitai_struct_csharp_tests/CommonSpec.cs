using System;
using System.IO;

namespace Kaitai
{
    public class CommonSpec
    {
        public String SourceFile(string filename)
        {
            string testPath = Environment.GetEnvironmentVariable("CSHARP_TEST_SRC_PATH");
            if (testPath == null)
            {
                char slash = Path.DirectorySeparatorChar;
                testPath = ".." + slash + ".." + slash + ".." + slash + "src";
            }
            return Path.Combine(testPath, filename);
        }
    }
}
