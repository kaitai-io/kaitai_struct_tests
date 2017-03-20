using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecDocstringsDocref : CommonSpec
    {
        [Test]
        public void TestDocstringsDocref()
        {
            var r = DocstringsDocref.FromFile(SourceFile("fixed_struct.bin"));
    
        }
    }
}
