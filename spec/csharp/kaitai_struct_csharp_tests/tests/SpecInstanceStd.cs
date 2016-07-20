using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecInstanceStd : CommonSpec
    {
        [Test]
        public void TestInstanceStd()
        {
            InstanceStd r = InstanceStd.FromFile(SourceFile("str_encodings.bin"));

            Assert.AreEqual(r.Header, "Some ");
        }
    }
}