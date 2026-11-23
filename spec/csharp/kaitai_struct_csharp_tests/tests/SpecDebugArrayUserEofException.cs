using NUnit.Framework;
using System.IO;

namespace Kaitai
{
    [TestFixture]
    public class SpecDebugArrayUserEofException : CommonSpec
    {
        [Test]
        public void TestDebugArrayUserEofException()
        {
            var r = DebugArrayUserEofException.FromFile(SourceFile("nav_parent_codes.bin"));

            Assert.Throws<EndOfStreamException>(
                delegate
                {
                    // --debug implies --no-auto-read
                    r._read();
                }
            );

            Assert.AreEqual(3, r.OneCat.Meow);
            Assert.AreEqual(73, r.OneCat.Chirp);
            Assert.AreEqual(3, r.ArrayOfCats.Count);
            Assert.AreEqual(49, r.ArrayOfCats[0].Meow);
            Assert.AreEqual(50, r.ArrayOfCats[0].Chirp);
            Assert.AreEqual(51, r.ArrayOfCats[1].Meow);
            Assert.AreEqual(66, r.ArrayOfCats[1].Chirp);
            Assert.AreEqual(98, r.ArrayOfCats[2].Meow);
        }
    }
}
