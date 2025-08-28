using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecDebugArrayUserCurrentExcluded : CommonSpec
    {
        [Test]
        public void TestDebugArrayUserCurrentExcluded()
        {
            var r = DebugArrayUserCurrentExcluded.FromFile(SourceFile("term_strz.bin"));

            // --debug implies --no-auto-read
            r._read();

            Assert.AreEqual(r.ArrayOfCats[0].Meow, new byte[] { 102, 111, 111 });
            Assert.AreEqual(r.ArrayOfCats[1].Meow, new byte[] { 124, 98 });
            Assert.AreEqual(r.ArrayOfCats[2].Meow, new byte[] { 97 });
        }
    }
}
