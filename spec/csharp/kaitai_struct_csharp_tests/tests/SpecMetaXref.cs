using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecMetaXref : CommonSpec
    {
        [Test]
        public void TestMetaXref()
        {
            var r = MetaXref.FromFile(SourceFile("fixed_struct.bin"));
        }
    }
}
