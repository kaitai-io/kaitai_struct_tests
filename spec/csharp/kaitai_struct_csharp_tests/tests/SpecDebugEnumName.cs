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

            Assert.AreEqual(r.One, DebugEnumName.TestEnum1.EnumValue80);
            Assert.AreEqual(r.ArrayOfInts[0], DebugEnumName.TestEnum2.EnumValue65);
            Assert.AreEqual(r.TestType.Field1, DebugEnumName.TestSubtype.InnerEnum1.EnumValue67);
            Assert.AreEqual(r.TestType.InstanceField, DebugEnumName.TestSubtype.InnerEnum2.EnumValue11);
        }
    }
}
