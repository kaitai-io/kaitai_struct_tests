using System;
using System.IO;

namespace Kaitai
{
    public class CommonSpec
    {
        public String SourceFile(string filename)
        {
            var currentDir = Path.GetDirectoryName(typeof(CommonSpec).Assembly.Location);
            return Path.Combine(currentDir, "src", filename);
        }
    }
}
