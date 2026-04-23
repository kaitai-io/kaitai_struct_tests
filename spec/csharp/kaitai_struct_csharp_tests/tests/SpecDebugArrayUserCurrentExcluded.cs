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

            Assert.AreEqual(new byte[] { 102, 111, 111 }, r.ArrayOfCats[0].Meow);
            Assert.AreEqual(new byte[] { 124, 98 }, r.ArrayOfCats[1].Meow);
            Assert.AreEqual(new byte[] { 97 }, r.ArrayOfCats[2].Meow);
        }
    }
}
