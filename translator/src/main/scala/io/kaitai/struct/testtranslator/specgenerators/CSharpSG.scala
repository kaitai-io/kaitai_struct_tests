package io.kaitai.struct.testtranslator.specgenerators

import _root_.io.kaitai.struct.{ClassTypeProvider, Utils}
import _root_.io.kaitai.struct.datatype.{DataType, EndOfStreamError, KSError}
import _root_.io.kaitai.struct.datatype.DataType.{EnumType, IntType}
import _root_.io.kaitai.struct.exprlang.Ast
import _root_.io.kaitai.struct.languages.CSharpCompiler
import _root_.io.kaitai.struct.testtranslator.{Main, TestAssert, TestEquals, TestSpec}
import _root_.io.kaitai.struct.translators.CSharpTranslator

class CSharpSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  val className = CSharpCompiler.type2class(spec.id)
  val translator = new CSharpTranslator(provider, importList)

  importList.add("NUnit.Framework")

  override def fileName(name: String): String = s"kaitai_struct_csharp_tests/tests/Spec$className.cs"

  override def header(): Unit = {
    out.puts("namespace Kaitai")
    out.puts("{")
    out.inc
    out.puts("[TestFixture]")
    out.puts(s"public class Spec$className : CommonSpec")
    out.puts("{")
    out.inc
    out.puts("[Test]")
    out.puts(s"public void Test$className()")
    out.puts("{")
    out.inc
  }

  override def runParse(): Unit = {
    out.puts(s"var r = $className.FromFile(SourceFile(" + "\"" + spec.data + "\"));")
  }

  override def runParseExpectError(exception: KSError): Unit = {
    exceptionToImports(exception)
    out.puts(s"Assert.Throws<${CSharpCompiler.ksErrorName(exception)}>(")
    out.inc
    out.puts("delegate")
    out.puts("{")
    out.inc
    out.puts(s"""$className.FromFile(SourceFile("${spec.data}"));""")
    out.dec
    out.puts("}")
    out.dec
    out.puts(");")
  }

  override def footer(): Unit = {
    out.dec
    out.puts("}")
    out.dec
    out.puts("}")
    out.dec
    out.puts("}")
  }

  override def simpleEquality(check: TestEquals): Unit = {
    val actStr = translateAct(check.actual)
    var expStr = translator.translate(check.expected)

    // Specially for enums with is tested for unknown values
    val actType = translator.detectType(check.actual)
    val expType = translator.detectType(check.expected)

    // If type of value is enum but we check it against numeric value,
    // do the cast of expected value to enum
    expStr = (actType, expType) match {
      case (act: EnumType, _: IntType) => {
        val casted = CSharpCompiler.kaitaiType2NativeType(importList, act, true)
        s"($casted) $expStr"
      }
      case _ => expStr
    }
    // TODO: fix order - actually it is (expected, actual)
    out.puts(s"Assert.AreEqual($actStr, $expStr);")
  }

  override def floatEquality(check: TestEquals): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    // TODO: fix order - actually it is (expected, actual)
    out.puts(s"Assert.AreEqual($actStr, $expStr, $FLOAT_DELTA);")
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    val actStr = translateAct(actual)
    out.puts(s"Assert.IsNull($actStr);")
  }

  override def trueArrayEquality(check: TestEquals, elType: DataType, elts: Seq[Ast.expr]): Unit = {
    simpleEquality(check) // FIXME
  }

  override def testException(actual: Ast.expr, exception: KSError): Unit = {
    exceptionToImports(exception)
    out.puts(s"Assert.Throws<${CSharpCompiler.ksErrorName(exception)}>(")
    out.inc
    out.puts("delegate")
    out.puts("{")
    out.inc
    out.puts(s"_ = ${translateAct(actual)};")
    out.dec
    out.puts("}")
    out.dec
    out.puts(");")
  }

  override def indentStr: String = "    "

  override def results: String = {
    "// " + AUTOGEN_COMMENT + "\n\n" +
      importList.toList.map((x) => s"using $x;").mkString("", "\n", "\n\n") +
      out.result
  }

  def translateAct(x: Ast.expr) =
    translator.translate(x).replace(Utils.capitalize(Main.INIT_OBJ_NAME), "r")

  def exceptionToImports(exception: KSError): Unit = {
    if (exception == EndOfStreamError)
      importList.add("System.IO")
  }
}
