package io.kaitai.struct.testtranslator.specgenerators

import io.kaitai.struct.datatype.DataType
import io.kaitai.struct.datatype.DataType.{ArrayType, BytesType}
import io.kaitai.struct.exprlang.Ast
import io.kaitai.struct.testtranslator.{TestAssert, TestSpec}
import io.kaitai.struct.translators.{AbstractTranslator, TypeDetector}

abstract class BaseGenerator(spec: TestSpec) extends SpecGenerator {
  val translator: AbstractTranslator with TypeDetector

  def header(): Unit
  def footer(): Unit

  def simpleAssert(check: TestAssert): Unit
  def nullAssert(actual: Ast.expr): Unit
  def trueArrayAssert(check: TestAssert, elType: DataType, elts: Seq[Ast.expr]): Unit
  def noAsserts(): Unit = {}

  override def run(): Unit = {
    header()

    spec.asserts.foreach { (check) =>
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
          simpleAssert(check)
      }
    }

    if (spec.asserts.isEmpty)
      noAsserts()

    footer()
  }
}
