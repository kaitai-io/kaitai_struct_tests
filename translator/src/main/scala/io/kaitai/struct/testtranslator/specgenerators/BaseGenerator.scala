package io.kaitai.struct.testtranslator.specgenerators

import _root_.io.kaitai.struct.datatype.{DataType, KSError}
import _root_.io.kaitai.struct.datatype.DataType.{ArrayType, BytesType, FloatType}
import _root_.io.kaitai.struct.exprlang.Ast
import _root_.io.kaitai.struct.testtranslator.{TestAssert, TestSpec}
import _root_.io.kaitai.struct.translators.{AbstractTranslator, TypeDetector}

abstract class BaseGenerator(spec: TestSpec) extends SpecGenerator {
  val translator: AbstractTranslator with TypeDetector

  val FLOAT_DELTA = "1e-6"

  def header(): Unit

  def runParse(): Unit = {}

  def runParseExpectError(exception: KSError): Unit = ???

  def footer(): Unit

  def simpleAssert(check: TestAssert): Unit

  def floatAssert(check: TestAssert): Unit = simpleAssert(check)

  def nullAssert(actual: Ast.expr): Unit

  def trueArrayAssert(check: TestAssert, elType: DataType, elts: Seq[Ast.expr]): Unit

  def noAsserts(): Unit = {}

  override def run(): Unit = {
    header()

    spec.exception match {
      case None =>
        runParse()
        out.puts
        runAsserts()
      case Some(ex) =>
        runParseExpectError(ex)
    }

    footer()
  }

  def runAsserts(): Unit = {
    spec.asserts.foreach { check =>
      check.expected match {
        case Ast.expr.Name(Ast.identifier("null")) =>
          nullAssert(check.actual)
        case Ast.expr.List(elts) =>
          // array assert
          val actType = translator.detectType(check.actual)
          actType match {
            case bt: BytesType =>
              // byte array => simple assert
              simpleAssert(check)
            case at: ArrayType =>
              // true array assert
              trueArrayAssert(check, at.elType, elts)
          }
        case _ =>
          val actType = translator.detectType(check.actual)
          actType match {
            case _: FloatType =>
              floatAssert(check)
            case _ =>
              simpleAssert(check)
          }
      }
    }

    if (spec.asserts.isEmpty)
      noAsserts()
  }
}
