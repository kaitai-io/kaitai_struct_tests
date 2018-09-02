using System;

namespace Kaitai
{
    using NUnit.Framework;

    [TestFixture]
    public class SpecDebugArrayUser : CommonSpec
    {
        [Test]
        public void TestDebugArrayUser()
        {
            DebugArrayUser r = DebugArrayUser.FromFile(SourceFile("fixed_struct.bin"));
            r._read();
            Assert.AreEqual(r.OneCat.Meow, 0x50);
            Assert.AreEqual(r.ArraysOfCats[0].Meow, 0x41);
            Assert.AreEqual(r.ArraysOfCats[1].Meow, 0x43);
            Assert.AreEqual(r.ArraysOfCats[2].Meow, 0x4b);
        }
    }
}
