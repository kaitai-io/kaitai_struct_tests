package io.kaitai.struct.testtranslator.specgenerators

import _root_.io.kaitai.struct.datatype.DataType
import _root_.io.kaitai.struct.datatype.{EndOfStreamError, KSError, UndecidedEndiannessError, ValidationNotEqualError}
import _root_.io.kaitai.struct.datatype.DataType._
import _root_.io.kaitai.struct.exprlang.Ast
import _root_.io.kaitai.struct.languages.PerlCompiler
import _root_.io.kaitai.struct.testtranslator.{Main, TestAssert, TestEquals, TestSpec, ExpectedException}
import _root_.io.kaitai.struct.translators.PerlTranslator
import _root_.io.kaitai.struct.{ClassTypeProvider, RuntimeConfig}

class PerlSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  val className = PerlCompiler.type2class(spec.id)
  val translator = new PerlTranslator(provider, importList)

  importList.add("strict")
  importList.add("warnings")
  importList.add("base qw(Test::Class)")
  importList.add("Test::More")
  importList.add(className)
  if (spec.exception.isDefined) {
    importList.add("Test::Exception")
  }

  override def fileName(name: String): String = s"Test$className.t"

  override def header(): Unit = {
    out.puts(s"sub test_${spec.id}: Test(${if (spec.exception.isDefined) 1 else spec.asserts.length}) {")
    out.inc
  }

  override def runParse(): Unit = {
    out.puts(s"my $$r = $className->from_file('src/${spec.data}');")
  }

  override def runParseExpectError(expException: ExpectedException): Unit = {
    val exception = expException.exception
    val msg = exception match {
      case UndecidedEndiannessError => "Unable to decide on endianness"
      case EndOfStreamError => "Requested \\d+ bytes, but only \\d+ bytes available"
    }
    out.puts(s"""throws_ok { $className->from_file('src/${spec.data}') } '/^$msg/';""")
  }

  override def footer(): Unit = {
    out.dec
    out.puts("}")
    out.puts
    out.puts("Test::Class->runtests;")
  }

  override def simpleEquality(check: TestEquals): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    val loc = translator.detectType(check.expected) match {
      // we need to use numerical comparison for booleans, because string comparison does not work
      // for all falsy values
      case _: BooleanType => s"cmp_ok($actStr, '==', $expStr, 'Equals')"
      case _ => s"is($actStr, $expStr, 'Equals')"
    }
    out.puts(s"$loc;")
  }

  override def floatEquality(check: TestEquals): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"ok(abs($actStr - $expStr) < $FLOAT_DELTA, 'Approx equals');")
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    val actStr = translateAct(actual)
    out.puts(s"ok(!defined($actStr), 'nil');")
  }

  override def trueArrayEquality(check: TestEquals, elType: DataType, elts: Seq[Ast.expr]): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"is_deeply($actStr, $expStr, 'Equals');")
  }

  override def indentStr: String = "    "

  override def results: String = {
    "# " + AUTOGEN_COMMENT + "\n\n" +
      s"package spec::perl::Test$className;\n\n" +
      importList.toList.map((x) => s"use $x;").mkString("", "\n", "\n") + "\n" +
      out.result
  }

  def translateAct(x: Ast.expr) =
    translator.translate(x).replace(s"$$self->${Main.INIT_OBJ_NAME}()", "$r")
}
