using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecEnumForUnknownId : CommonSpec
    {
        [Test]
        public void TestEnumForUnknownId()
        {
            var r = EnumForUnknownId.FromFile(SourceFile("fixed_struct.bin"));
            Assert.AreEqual(r.One, (Kaitai.EnumForUnknownId.Animal) 80);
        }
    }
}
