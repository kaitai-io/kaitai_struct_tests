using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecEnumToIInvalid : CommonSpec
    {
        [Test]
        public void TestEnumToIInvalid()
        {
            var r = EnumToIInvalid.FromFile(SourceFile("term_strz.bin"));

            Assert.AreEqual(r.Pet1, EnumToIInvalid.Animal.Dog);
            Assert.AreEqual(r.Pet2, (EnumToIInvalid.Animal) 111);
            Assert.AreEqual(r.Pet2I, 111);
            Assert.AreEqual(r.Pet2IToS, "111");
            Assert.AreEqual(r.Pet2Mod, 32879);
            Assert.AreEqual(r.OneLtTwo, true);
            Assert.AreEqual(r.Pet2EqIntT, true);
            Assert.AreEqual(r.Pet2EqIntF, false);
        }
    }
}
