using System;

namespace Kaitai
{
    using NUnit.Framework;

    [TestFixture]
    public class Tests
    {
        String SRC_DIR = "src/";

        [Test]
        public void TestHelloWorld()
        {
            HelloWorld r = HelloWorld.fromFile(SRC_DIR + "fixed_struct.bin");
            Assert.AreEqual(r.One(), 0x50);
        }
    }
}
