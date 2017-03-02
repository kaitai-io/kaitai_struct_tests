using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecRepeatUntilSized : CommonSpec
    {
        [Test]
        public void TestRepeatUntilSized()
        {
            var r = RepeatUntilSized.FromFile(SourceFile("repeat_until_process.bin"));
            Assert.AreEqual(r.Records.Count, 3);
    
            Assert.AreEqual(r.Records[0].Marker, 0xe8);
            Assert.AreEqual(r.Records[0].Body, 0xaaaaaaba);
    
            Assert.AreEqual(r.Records[1].Marker, 0xfa);
            Assert.AreEqual(r.Records[1].Body, 0xaaaab89e);
    
            Assert.AreEqual(r.Records[2].Marker, 0xaa);
            Assert.AreEqual(r.Records[2].Body, 0x55555555);
        }
    }
}
