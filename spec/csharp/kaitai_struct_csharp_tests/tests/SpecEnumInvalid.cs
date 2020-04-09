using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecEnumInvalid : CommonSpec
    {
        [Test]
        public void TestEnumInvalid()
        {
            var r = EnumInvalid.FromFile(SourceFile("term_strz.bin"));

            Assert.AreEqual(r.Pet1, EnumInvalid.Animal.Dog);
            Assert.AreEqual(r.Pet2, (EnumInvalid.Animal) 111);
        }
    }
}
