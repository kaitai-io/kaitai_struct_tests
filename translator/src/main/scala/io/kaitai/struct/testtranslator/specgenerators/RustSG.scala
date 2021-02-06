package io.kaitai.struct.testtranslator.specgenerators

import _root_.io.kaitai.struct.testtranslator.{Main, TestAssert, TestSpec}
import io.kaitai.struct.datatype.DataType
import io.kaitai.struct.datatype.DataType.{EnumType, SwitchType}
import io.kaitai.struct.exprlang.Ast
import io.kaitai.struct.exprlang.Ast.expr.{Attribute, CastToType, Subscript}
import io.kaitai.struct.format.NamedIdentifier
import io.kaitai.struct.languages.RustCompiler
import io.kaitai.struct.translators.RustTranslator
import io.kaitai.struct.{ClassTypeProvider, RuntimeConfig}

class RustSG(spec: TestSpec, provider: ClassTypeProvider)
    extends BaseGenerator(spec) {
  val className: String = RustCompiler.type2class(spec.id)
  val translator = new RustTranslator(provider, RuntimeConfig())

  override def fileName(name: String): String = s"tests/test_$name.rs"

  override def header(): Unit = {
    out.puts("use kaitai::{BytesReader, KStruct, KStructUnit};")
    out.puts(s"use kaitai_test::${spec.id}::*;")
    out.puts(s"use std::fs;")
    out.puts

    out.puts("#[test]")
    out.puts(s"fn test_${spec.id}() {")
    out.inc

    out.puts(
      "let data = fs::read(\"../../src/" + spec.data + "\").expect(\"Unable to read data.\");"
    )
    out.puts("let reader = BytesReader::new(&data[..]);")
    out.puts(s"let mut r = $className::default();")
    out.puts(
      "r.read(&reader, None, KStructUnit::parent_stack()).expect(\"Unable to parse data.\");"
    )
  }

  override def footer(): Unit = {
    out.dec
    out.puts("}")
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    out.puts(s"// nullAsset(${translateActual(actual)});")
  }

  override def trueArrayAssert(check: TestAssert,
                               elType: DataType,
                               elts: Seq[Ast.expr]): Unit = {
    simpleAssert(check)
  }

  override def simpleAssert(check: TestAssert): Unit = {

    // TODO: Fix attribute chaining
    check.actual match {
      case attribute: Attribute if attribute.value.isInstanceOf[Attribute] =>
        out.puts(s"// assert_eq!(${check.actual}, ${check.expected})")
        return
      case attribute: Attribute if attribute.value.isInstanceOf[Subscript] =>
        out.puts(s"// assert_eq!(${check.actual}, ${check.expected})")
        return
      case attribute: Attribute if attribute.value.isInstanceOf[CastToType] =>
        out.puts(s"// assert_eq!(${check.actual}, ${check.expected})")
        return
      case _ => ()
    }

    // TODO: Fix instance unwrapping
    val attrName = check.actual.asInstanceOf[Attribute].attr
    val classSpec = provider.resolveClassSpec(provider.nowClass, spec.id)
    if (classSpec.instances.exists(i => i._1.name == attrName.name)) {
      out.puts(s"// assert_eq!(${check.actual}, ${check.expected})")
      return
    }

    // Normally you'd want to look up types through `translator.detectType`,
    // but that doesn't properly handle things like SwitchType
    // TODO: Fix enum equality checks
    val attrType = classSpec.seq.find(a => a.id == NamedIdentifier(attrName.name))
    attrType match {
      case Some(a) if a.dataType.isInstanceOf[EnumType] || a.dataType.isInstanceOf[SwitchType] =>
        out.puts(s"// assert_eq!(${check.actual}, ${check.expected})")
        return
      case _ => ()
    }

    out.puts(s"assert_eq!(${translateActual(check.actual)}, ${translator.translate(check.expected)});")
  }

  def translateActual(actual: Ast.expr): String =
    // Because we will try to look up attributes as if they're on `Main.INIT_OBJ_NAME`,
    // trim that out of the lookup path
    translator.translate(actual).replace("self." + Main.INIT_OBJ_NAME, "r")

  override def indentStr: String = "    "

  override def results: String = "// " + AUTOGEN_COMMENT + "\n\n" + out.result

  def actualIsOptional(x: Ast.expr): Boolean = {
    // TODO: Need a much more robust way to check for `Option<>` types
    val attrName = x match {
      case a: Attribute => a.attr
      case _            => return false
    }

    // If it's an instance, it's guaranteed to be optional
    if (provider
          .resolveClassSpec(provider.nowClass, spec.id)
          .instances
          .keys
          .exists(p => p.name == attrName.name))
      return true

    // Look up the type from the struct definition; `detectType` doesn't
    // properly handle `SwitchType` for example
    val seqType = provider
      .resolveClassSpec(provider.nowClass, spec.id)
      .seq
      .filter(a => a.id == NamedIdentifier(attrName.name))
      .head

    seqType.dataType match {
      case _: EnumType | _: SwitchType => return true
      case _ => ()
    }

    false
  }
}
