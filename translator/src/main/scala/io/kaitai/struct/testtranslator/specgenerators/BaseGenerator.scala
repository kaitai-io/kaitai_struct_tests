package io.kaitai.struct.testtranslator.specgenerators

import _root_.io.kaitai.struct.datatype.{DataType, KSError}
import _root_.io.kaitai.struct.datatype.DataType.{ArrayType, BytesType, FloatType}
import _root_.io.kaitai.struct.exprlang.Ast
import _root_.io.kaitai.struct.testtranslator.{TestAssert, TestEquals, TestException, TestSpec, ExpectedException}
import _root_.io.kaitai.struct.translators.{AbstractTranslator, TypeDetector}

abstract class BaseGenerator(spec: TestSpec) extends SpecGenerator {
  val translator: AbstractTranslator with TypeDetector

  val FLOAT_DELTA = "1e-6"

  def header(): Unit

  def runParse(): Unit = {}

  def runParseExpectError(expException: ExpectedException): Unit = ???

  def footer(): Unit

  def simpleEquality(check: TestEquals): Unit

  def floatEquality(check: TestEquals): Unit = simpleEquality(check)

  def nullAssert(actual: Ast.expr): Unit

  def trueArrayEquality(check: TestEquals, elType: DataType, elts: Seq[Ast.expr]): Unit

  def testException(actual: Ast.expr, exception: KSError): Unit = ???

  def noAsserts(): Unit = {}

  override def run(): Unit = {
    header()

    spec.exception match {
      case None =>
        runParse()
        runAsserts()
      case Some(expException) =>
        runParseExpectError(expException)
    }

    footer()
  }

  def runAsserts(): Unit = {
    spec.asserts.foreach { assert =>
      assert match {
        case check: TestEquals =>
          check.expected match {
            case Ast.expr.Name(Ast.identifier("null")) =>
              nullAssert(check.actual)
            case Ast.expr.List(elts) =>
              // array assert
              val actType = translator.detectType(check.actual)
              actType match {
                case bt: BytesType =>
                  // byte array => simple assert
                  simpleEquality(check)
                case at: ArrayType =>
                  // true array assert
                  trueArrayEquality(check, at.elType, elts)
              }
            case _ =>
              val actType = translator.detectType(check.actual)
              actType match {
                case _: FloatType =>
                  floatEquality(check)
                case _ =>
                  simpleEquality(check)
              }
          }
        case TestException(actual, exception) =>
          testException(actual, exception)
      }
    }

    if (spec.asserts.isEmpty)
      noAsserts()
  }
}
