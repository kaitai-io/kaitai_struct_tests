using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecDebugArrayUser : CommonSpec
    {
        [Test]
        public void TestDebugArrayUser()
        {
            var r = DebugArrayUser.FromFile(SourceFile("fixed_struct.bin"));

            // --debug implies --no-auto-read
            r._read();

            Assert.AreEqual(r.OneCat.Meow, 80);
            Assert.AreEqual(r.ArrayOfCats.Count, 3);
            Assert.AreEqual(r.ArrayOfCats[0].Meow, 65);
            Assert.AreEqual(r.ArrayOfCats[1].Meow, 67);
            Assert.AreEqual(r.ArrayOfCats[2].Meow, 75);
        }
    }
}
