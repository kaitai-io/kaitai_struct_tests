// Autogenerated from KST: please remove this line if doing any edits by hand!

using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecValidFailRepeatContents : CommonSpec
    {
        [Test]
        public void TestValidFailRepeatContents()
        {
            Assert.Throws<ValidationNotEqualError>(
                delegate
                {
                    ValidFailRepeatContents.FromFile(SourceFile("bcd_user_type_be.bin"));
                }
            );
        }
    }
}
