package io.kaitai.struct.testtranslator

import java.io.{File, FileReader, FileWriter}

import io.kaitai.struct.{ClassTypeProvider, ConstructClassCompiler}
import io.kaitai.struct.JavaMain.CLIConfig
import io.kaitai.struct.datatype.DataType.UserTypeInstream
import io.kaitai.struct.format._
import io.kaitai.struct.formats.JavaKSYParser
import io.kaitai.struct.languages._
import io.kaitai.struct.languages.components.LanguageCompilerStatic
import io.kaitai.struct.testtranslator.specgenerators._

object Main extends App {
  val baseDir = ".."
  val specKsDir = s"$baseDir/spec/ks"
  val formatsDir = s"$baseDir/formats"
  val outDir = s"$specKsDir/out"

  def writeFile(fileName: String, contents: String): Unit = {
    val outPath = new File(fileName)

    // Ensure that all directories leading to this path exist
    val parentPath = outPath.getParentFile
    parentPath.mkdirs

    val fw = new FileWriter(outPath)
    fw.write(contents)
    fw.close()
  }

  def loadTestSpec(testName: String): TestSpec = {
    val reader = new FileReader(s"$specKsDir/$testName.kst")
    val src = JavaKSYParser.readerToYaml(reader)
    TestSpec.fromYaml(src)
  }

  final val INIT_OBJ_TYPE = "unique_top_level_container"
  final val INIT_OBJ_NAME = "q1w2e3"

  def loadClassSpecs(testName: String): ClassSpecs = {
    val origSpecs = JavaKSYParser.localFileToSpecs(s"$formatsDir/$testName.ksy", CLIConfig())

    val userType = UserTypeInstream(origSpecs.firstSpec.name, None)
    userType.classSpec = Some(origSpecs.firstSpec)

    val initObj = ClassSpec(
      path = List(),
      isTopLevel = true,
      meta = MetaSpec(
        path = List("meta"),
        isOpaque = false,
        id = Some(INIT_OBJ_TYPE),
        endian = None,
        encoding = None,
        forceDebug = false,
        opaqueTypes = None,
        imports = List()
      ),
      DocSpec.EMPTY,
      params = List(),
      seq = List(
        AttrSpec(
          path = List("seq", "0"),
          id = NamedIdentifier(INIT_OBJ_NAME),
          dataType = userType
        )
      ),
      types = Map(),
      instances = Map(),
      enums = Map()
    )

    initObj.name = List(INIT_OBJ_TYPE)

    origSpecs.put(INIT_OBJ_TYPE, initObj)

    origSpecs
  }

  def getSG(lang: LanguageCompilerStatic, testSpec: TestSpec, provider: ClassTypeProvider): SpecGenerator = {
    lang match {
      case ConstructClassCompiler => new ConstructSG(testSpec, provider)
      case CppCompiler => new CppStlSG(testSpec, provider)
      case GoCompiler => new GoSG(testSpec, provider)
      case JavaCompiler => new JavaSG(testSpec, provider)
      case JavaScriptCompiler => new JavaScriptSG(testSpec, provider)
      case PHPCompiler => new PHPSG(testSpec, provider)
      case PythonCompiler => new PythonSG(testSpec, provider)
      case RubyCompiler => new RubySG(testSpec, provider)
    }
  }

  final val ALL_LANGS = List(
    ConstructClassCompiler,
    CppCompiler,
    GoCompiler,
    JavaCompiler,
    JavaScriptCompiler,
    PHPCompiler,
    PythonCompiler,
    RubyCompiler,
  )

  def doOneSpec(lang: LanguageCompilerStatic, testName: String, testSpec: TestSpec, provider: ClassTypeProvider) = {
    val langName = LanguageCompilerStatic.CLASS_TO_NAME(lang)
    val sg = getSG(lang, testSpec, provider)
    try {
      sg.run()
      writeFile(s"$outDir/$langName/${sg.fileName(testName)}", sg.results)
    } catch {
      case e: Throwable => e.printStackTrace(Console.err)
    }
  }

  def doOneSpecOneLang(lang: LanguageCompilerStatic, testName: String) {
    val testSpec = loadTestSpec(testName)
    val classSpecs = loadClassSpecs(testName)
    val initObj = classSpecs(INIT_OBJ_TYPE)
    val provider = new ClassTypeProvider(classSpecs, initObj)

    doOneSpec(lang, testName, testSpec, provider)
  }

  def doOneSpecAllLangs(testName: String): Unit = {
    Console.println(s"doOneSpecAllLangs($testName)")

    val testSpec = loadTestSpec(testName)
    val classSpecs = loadClassSpecs(testName)
    val initObj = classSpecs(INIT_OBJ_TYPE)
    val provider = new ClassTypeProvider(classSpecs, initObj)

    ALL_LANGS.foreach((lang) => doOneSpec(lang, testName, testSpec, provider))
  }

  def doAllSpecsAllLangs(): Unit = {
    val dir = new File(specKsDir)
    dir.list().foreach((fn) =>
      if (fn.endsWith(".kst"))
        doOneSpecAllLangs(fn.substring(0, fn.length - 4))
    )
  }

  //doOneSpecAllLangs("hello_world")
  //doOneSpecAllLangs("process_xor4_const")
  //doOneSpecAllLangs("enum_0")
  //doOneSpecOneLangs(CppCompiler, "switch_manual_int_else")
  //doOneSpecAllLangs("if_values")
  //doOneSpecOneLang(GoCompiler, "type_ternary")

  doAllSpecsAllLangs()
}
