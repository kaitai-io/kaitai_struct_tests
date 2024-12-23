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

            Assert.AreEqual(r.OneCat.Meow, 3);
            Assert.AreEqual(r.OneCat.Chirp, 73);
            Assert.AreEqual(r.ArrayOfCats.Count, 3);
            Assert.AreEqual(r.ArrayOfCats[0].Meow, 49);
            Assert.AreEqual(r.ArrayOfCats[0].Chirp, 50);
            Assert.AreEqual(r.ArrayOfCats[1].Meow, 51);
            Assert.AreEqual(r.ArrayOfCats[1].Chirp, 66);
            Assert.AreEqual(r.ArrayOfCats[2].Meow, 98);
        }
    }
}
