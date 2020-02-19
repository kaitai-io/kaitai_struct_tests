using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecRepeatEosU4 : CommonSpec
    {
        [Test]
        public void TestRepeatEosU4()
        {
            RepeatEosU4 r = RepeatEosU4.FromFile(SourceFile("repeat_eos_struct.bin"));

            CollectionAssert.AreEqual(r.Numbers, new long[] {0, 0x42, 0x42, 0x815});
        }
    }
}