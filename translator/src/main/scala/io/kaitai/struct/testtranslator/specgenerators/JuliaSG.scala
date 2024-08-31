package io.kaitai.struct.testtranslator.specgenerators

import _root_.io.kaitai.struct.ClassTypeProvider
import _root_.io.kaitai.struct.datatype.{DataType, KSError}
import _root_.io.kaitai.struct.exprlang.Ast
import _root_.io.kaitai.struct.languages.JuliaCompiler
import _root_.io.kaitai.struct.testtranslator.{Main, TestEquals, TestSpec}
import _root_.io.kaitai.struct.translators.JuliaTranslator

class JuliaSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  importList.add("using Test")

  val translator = new JuliaTranslator(provider, importList)
  val className = JuliaCompiler.type2class(spec.id)

  override def fileName(name: String): String = s"test_$name.jl"

  override def indentStr: String = "    "

  override def header(): Unit = {
    importList.add(s"import ${className}Module")
    out.puts
    out.puts(s"@testset ${'"'}$className test${'"'} begin")
    out.inc
  }

  override def runParse(): Unit = {
    out.puts(s"r = ${className}Module.from_file(${'"'}src/${spec.data}${'"'})")
    out.puts
  }

  override def runParseExpectError(exception: KSError): Unit = {
    importList.add("using KaitaiStruct")
    out.puts(s"@test_throws ${JuliaCompiler.ksErrorName(exception)} ${className}Module.from_file(${'"'}src/${spec.data}${'"'})")
  }

  override def footer(): Unit = {
    out.dec
    out.puts("end")
  }

  override def simpleEquality(check: TestEquals): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translateExp(check.expected)
    out.puts(s"@test $actStr == $expStr")
  }

  override def floatEquality(check: TestEquals): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"@test $actStr ≈ $expStr atol=10^(-6)")
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    val actStr = translateAct(actual)
    out.puts(s"@test $actStr === nothing")
  }

  override def trueArrayEquality(check: TestEquals, elType: DataType, elts: Seq[Ast.expr]): Unit =
    simpleEquality(check)

  override def testException(actual: Ast.expr, exception: KSError): Unit = {
    importList.add("using KaitaiStruct")
    out.puts(s"@test_throws ${JuliaCompiler.ksErrorName(exception)} ${translateAct(actual)}")
  }

  override def noAsserts() =
    out.puts("")

  def translateAct(x: Ast.expr) =
    translator.translate(x).replace("this." + Main.INIT_OBJ_NAME, "r")

  def translateExp(x: Ast.expr) =
    translator.translate(x).replace("this._root", className)

  override def results: String =
    "# " + AUTOGEN_COMMENT + "\n\n" + super.results
}
