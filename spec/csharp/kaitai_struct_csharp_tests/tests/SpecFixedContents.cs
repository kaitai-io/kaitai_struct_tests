using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecFixedContents : CommonSpec
    {
        [Test]
        public void TestFixedContents()
        {
            var r = FixedContents.FromFile(SourceFile("fixed_struct.bin"));
        }
    }
}
