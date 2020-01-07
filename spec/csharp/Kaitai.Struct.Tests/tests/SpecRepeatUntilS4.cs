using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecRepeatUntilS4 : CommonSpec
    {
        [Test]
        public void TestRepeatUntilS4()
        {
            var r = RepeatUntilS4.FromFile(SourceFile("repeat_until_s4.bin"));
            Assert.AreEqual(r.Entries, new int[] { 0x42, 0x1337, -251658241, -1 });
            Assert.AreEqual(r.Afterall, "foobar");
        }
    }
}
