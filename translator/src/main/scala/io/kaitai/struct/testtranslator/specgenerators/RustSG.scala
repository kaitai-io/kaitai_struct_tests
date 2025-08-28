package io.kaitai.struct.testtranslator.specgenerators

import _root_.io.kaitai.struct.{ClassTypeProvider, JSON, RuntimeConfig}
import _root_.io.kaitai.struct.datatype.{DataType, EndOfStreamError, KSError, UndecidedEndiannessError, ValidationError}
import _root_.io.kaitai.struct.exprlang.Ast
import _root_.io.kaitai.struct.languages.RustCompiler
import _root_.io.kaitai.struct.testtranslator.{Main, TestAssert, TestEquals, TestSpec, ExpectedException}
import _root_.io.kaitai.struct.translators.RustTranslator
import _root_.io.kaitai.struct.datatype.DataType.{ArrayType, BooleanType, BytesType, EnumType, IntType, SwitchType, UserType}

class RustSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  val className: String = RustCompiler.type2class(spec.id)
  val translator = new RustTranslator(provider, RuntimeConfig())

  override def fileName(name: String): String = s"tests/test_$name.rs"

  override def header(): Unit = {
    importList.add("use std::fs;")
    importList.add("extern crate kaitai;")
    importList.add("use self::kaitai::*;")
    importList.add(s"use rust::formats::${spec.id}::*;")

    spec.extraImports.foreach(entry => importList.add(s"use rust::formats::$entry::*;"))

    out.puts("#[test]")
    out.puts(s"fn test_${spec.id}() -> KResult<()> {")
    out.inc
    out.puts(s"""let bytes = fs::read("../../src/${spec.data}").unwrap();""")
    out.puts("let _io = BytesReader::from(bytes);")
  }

  override def runParse(): Unit = {
    out.puts(s"let r: OptRc<$className> = $className::read_into(&_io, None, None)?;")
  }

  override def runParseExpectError(expException: ExpectedException): Unit = {
    val exception = expException.exception
    out.puts(s"let res: KResult<OptRc<$className>> = $className::read_into(&_io, None, None);")
    exceptionAssert("res", exception)
  }

  override def footer(): Unit = {
    out.puts("Ok(())")
    out.dec
    out.puts("}")
  }

  override def simpleEquality(check: TestEquals): Unit = {
    val actType = translator.detectType(check.actual)
    var actStr = translateAct(check.actual)
    val expType = translator.detectType(check.expected)
    var expStr = translator.translate(check.expected)
    (actType, expType) match {
      case (_: EnumType, _: IntType) =>
        out.puts(s"let n: i64 = (&$actStr).into();")
        actStr = s"n"
      case _ =>
    }
    out.puts(s"assert_eq!($actStr, $expStr);")
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    val actStr = translator.remove_deref(translateAct(actual))
    out.puts(s"assert!($actStr.is_none());")
  }

  override def trueArrayEquality(check: TestEquals, elType: DataType, elts: Seq[Ast.expr]): Unit = {
    simpleEquality(check)
  }

  override def testException(actual: Ast.expr, exception: KSError): Unit = {
    val actStr = translateAct(actual)
    require(actStr.endsWith(")?"), s"$actStr does not end with )?")
    val s = translator.remove_deref(actStr.stripSuffix(")?") + ")")
    exceptionAssert(s, exception)
  }

  override def indentStr: String = "    "

  override def results: String = {
    "// " + AUTOGEN_COMMENT + "\n\n" +
      importList.toList.mkString("", "\n", "\n") + "\n" +
      out.result
  }

  def translateAct(x: Ast.expr): String =
    translator.translate(x).replace(s"self.${Main.INIT_OBJ_NAME}()", "r")

  private def exceptionAssert(actualStr: String, exception: KSError): Unit = {
    out.puts(s"""let err = $actualStr.expect_err("expected Err representing $exception, but got Ok");""")
    val errorName = RustCompiler.ksErrorName(exception)
    val expectedErrPattern = exception match {
      case EndOfStreamError | UndecidedEndiannessError =>
        s"$errorName { .. }"
      case err: ValidationError =>
        val validationKind = RustCompiler.validationErrorKind(err)
        s"$errorName(ValidationFailedError { kind: $validationKind, .. })"
      case _ =>
        out.puts(s"assert_eq!(err, $errorName);")
        return
    }
    val expectedErrPatternStr = translator.translate(Ast.expr.Str(expectedErrPattern))
    out.puts("assert!(")
    out.inc
    out.puts(s"matches!(err, $expectedErrPattern),")
    out.puts("\"expected: {}\\n but got: {:?}\",")
    out.puts(s"$expectedErrPatternStr, err")
    out.dec
    out.puts(");")
  }
}
