using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecNavParentSwitch : CommonSpec
    {
        [Test]
        public void TestNavParentSwitch()
        {
            var r = NavParentSwitch.FromFile(SourceFile("nav_parent_switch.bin"));
            Assert.AreEqual(r.Category, 1);
            Assert.AreEqual(r.Content.Foo, 0x42);
            Assert.AreEqual(r.Content.Subelement.Bar, 0xff);
        }
    }
}
