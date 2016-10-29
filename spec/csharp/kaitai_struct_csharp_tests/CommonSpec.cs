using System;
using System.IO;

namespace Kaitai
{
    public class CommonSpec
    {
        public String SourceFile(string filename)
        {
            var currentDir = Path.GetDirectoryName(typeof(CommonSpec).Assembly.Location);
            #if __MonoCS__
                currentDir = Environment.CurrentDirectory;
            #endif
            return Path.Combine(currentDir, "src", filename);
        }
    }
}
