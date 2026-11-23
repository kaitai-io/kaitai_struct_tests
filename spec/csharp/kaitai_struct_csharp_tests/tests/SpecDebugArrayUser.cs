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

            Assert.AreEqual(80, r.OneCat.Meow);
            Assert.AreEqual(3, r.ArrayOfCats.Count);
            Assert.AreEqual(65, r.ArrayOfCats[0].Meow);
            Assert.AreEqual(67, r.ArrayOfCats[1].Meow);
            Assert.AreEqual(75, r.ArrayOfCats[2].Meow);
        }
    }
}
