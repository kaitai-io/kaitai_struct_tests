using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecUserType : CommonSpec
    {
        [Test]
        public void TestUserType()
        {
            var r = UserType.FromFile(SourceFile("repeat_until_s4.bin"));
            Assert.AreEqual(r.One.Width, 0x42);
            Assert.AreEqual(r.One.Height, 0x1337);
        }
    }
}
