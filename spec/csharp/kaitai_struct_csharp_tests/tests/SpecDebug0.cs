using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecDebug0 : CommonSpec
    {
        [Test]
        public void TestDebug0()
        {
            var r = Debug0.FromFile(SourceFile("fixed_struct.bin"));

            // --debug implies --no-auto-read
            r._read();

            Assert.AreEqual(r.One, 80);
            Assert.AreEqual(r.ArrayOfInts.Count, 3);
            Assert.AreEqual(r.ArrayOfInts[0], 65);
            Assert.AreEqual(r.ArrayOfInts[1], 67);
            Assert.AreEqual(r.ArrayOfInts[2], 75);
            Assert.AreEqual(r.Unnamed_2, 45);

            // FIXME: also test --read-pos once it is implemented
        }
    }
}
