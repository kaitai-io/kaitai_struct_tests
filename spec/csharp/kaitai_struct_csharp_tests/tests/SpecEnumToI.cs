using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecEnumToI : CommonSpec
    {
        [Test]
        public void TestEnumToI()
        {
            var r = EnumToI.FromFile(SourceFile("enum_0.bin"));
            Assert.AreEqual(r.Pet1, EnumToI.Animal.Cat);
            Assert.AreEqual(r.Pet2, EnumToI.Animal.Chicken);
    
            Assert.AreEqual(r.Pet1I, 4);
            Assert.AreEqual(r.OneLtTwo, true);
        }
    }
}
