using NUnit.Framework;

namespace Kaitai
{
    [TestFixture]
    public class SpecDebugEnumName : CommonSpec
    {
        [Test]
        public void TestDebugEnumName()
        {
            var r = DebugEnumName.FromFile(SourceFile("fixed_struct.bin"));

            // --debug implies --no-auto-read
            r._read();

            Assert.AreEqual(DebugEnumName.TestEnum1.EnumValue80, r.One);
            Assert.AreEqual(DebugEnumName.TestEnum2.EnumValue65, r.ArrayOfInts[0]);
            Assert.AreEqual(DebugEnumName.TestSubtype.InnerEnum1.EnumValue67, r.TestType.Field1);
            Assert.AreEqual(DebugEnumName.TestSubtype.InnerEnum2.EnumValue11, r.TestType.InstanceField);
        }
    }
}
