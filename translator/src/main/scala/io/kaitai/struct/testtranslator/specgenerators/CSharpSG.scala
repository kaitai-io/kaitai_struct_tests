package io.kaitai.struct.testtranslator.specgenerators

import io.kaitai.struct.{ClassTypeProvider, Utils}
import io.kaitai.struct.datatype.DataType
import io.kaitai.struct.exprlang.Ast
import io.kaitai.struct.languages.CSharpCompiler
import io.kaitai.struct.testtranslator.{Main, TestAssert, TestSpec}
import io.kaitai.struct.translators.CSharpTranslator

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
    out.puts(s"var r = $className.FromFile(SourceFile(" + "\"" + spec.data + "\"));")
    out.puts
  }

  override def footer(): Unit = {
    out.dec
    out.puts("}")
    out.dec
    out.puts("}")
    out.dec
    out.puts("}")
  }

  override def simpleAssert(check: TestAssert): Unit = {
    val actType = translator.detectType(check.actual)
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    actType match {
      case _ =>
        // TODO: fix order - actually it is (expected, actual)
        out.puts(s"Assert.AreEqual($actStr, $expStr);")
    }
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    val actStr = translateAct(actual)
    out.puts(s"Assert.IsNull($actStr);")
  }

  override def trueArrayAssert(check: TestAssert, elType: DataType, elts: Seq[Ast.expr]): Unit = {
    simpleAssert(check) // FIXME
  }

  override def indentStr: String = "    "

  override def results: String = {
    "// " + AUTOGEN_COMMENT + "\n\n" +
      importList.toList.map((x) => s"using $x;").mkString("", "\n", "\n\n") +
      out.result
  }

  def translateAct(x: Ast.expr) =
    translator.translate(x).replace(Utils.capitalize(Main.INIT_OBJ_NAME), "r")
}
