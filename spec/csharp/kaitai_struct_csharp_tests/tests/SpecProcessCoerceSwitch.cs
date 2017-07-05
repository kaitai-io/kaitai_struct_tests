using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecProcessCoerceSwitch : CommonSpec
    {
        [Test]
        public void TestProcessCoerceSwitch()
        {
            var r = ProcessCoerceSwitch.FromFile(SourceFile("process_coerce_switch.bin"));
            Assert.AreEqual(r.BufType, 0);
            Assert.AreEqual(r.Flag, 0);
            Assert.AreEqual(r.Buf.Foo, "AAAA");
        }
    }
}
