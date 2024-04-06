using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecParamsDef : CommonSpec
    {
        [Test]
        public void TestParamsDef()
        {
            var io = new KaitaiStream(SourceFile("term_strz.bin"));
            var r = new ParamsDef(5, true, io);

            AreEqual(r.Buf, "foo|b");
            AreEqual(r.Trailer, 0x61);
        }
    }
}
