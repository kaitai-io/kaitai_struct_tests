package io.kaitai.struct.testtranslator.specgenerators

import _root_.io.kaitai.struct.ClassTypeProvider
import _root_.io.kaitai.struct.datatype.{DataType, KSError}
import _root_.io.kaitai.struct.exprlang.Ast
import _root_.io.kaitai.struct.languages.JuliaCompiler
import _root_.io.kaitai.struct.testtranslator.{Main, TestAssert, TestSpec}
import _root_.io.kaitai.struct.translators.JuliaTranslator

class JuliaSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  importList.add("using Test")
  importList.add("using TestReports")

  val translator = new JuliaTranslator(provider, importList)
  val className = JuliaCompiler.type2class(spec.id)

  override def fileName(name: String): String = s"test_$name.jl"

  override def indentStr: String = "    "

  override def header(): Unit = {
    importList.add(s"include(${'"'}../../compiled/julia/${spec.id}.jl${'"'})")
    out.puts
    out.puts(s"@testset ${'"'}$className test${'"'} begin")
    out.inc
  }

  override def runParse(): Unit = {
    out.puts(s"r = from_file(${'"'}../../src/${spec.data}${'"'})")
    out.puts
  }

  override def runParseExpectError(exception: KSError): Unit = {
    out.puts(s"@test_throws ${JuliaCompiler.ksErrorName(exception)} from_file(${'"'}../../src/${spec.data}${'"'})")
  }

  override def footer(): Unit = {
    out.dec
    out.puts("end")
  }

  override def simpleAssert(check: TestAssert): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translateExp(check.expected)
    out.puts(s"@test $actStr == $expStr")
  }

  override def floatAssert(check: TestAssert): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"@test $actStr ≈ $expStr atol=10^(-6)")
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    val actStr = translateAct(actual)
    out.puts(s"@test $actStr === nothing")
  }

  override def trueArrayAssert(check: TestAssert, elType: DataType, elts: Seq[Ast.expr]): Unit =
    simpleAssert(check)

  override def noAsserts() =
    out.puts("")

  def translateAct(x: Ast.expr) =
    translator.translate(x).replace("this." + Main.INIT_OBJ_NAME, "r")

  def translateExp(x: Ast.expr) =
    translator.translate(x).replace("this._root", className)

  override def results: String =
    "# " + AUTOGEN_COMMENT + "\n\n" + super.results
}
