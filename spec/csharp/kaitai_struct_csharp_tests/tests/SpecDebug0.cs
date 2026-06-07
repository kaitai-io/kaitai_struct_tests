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

            Assert.AreEqual(80, r.One);
            Assert.AreEqual(3, r.ArrayOfInts.Count);
            Assert.AreEqual(65, r.ArrayOfInts[0]);
            Assert.AreEqual(67, r.ArrayOfInts[1]);
            Assert.AreEqual(75, r.ArrayOfInts[2]);
            Assert.AreEqual(45, r.Unnamed_2);

            // FIXME: also test --read-pos once it is implemented
        }
    }
}
