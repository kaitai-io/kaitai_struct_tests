package io.kaitai.struct.testtranslator

import java.io.{File, FileReader, FileWriter}

import io.kaitai.struct.JavaMain.CLIConfig
import io.kaitai.struct.datatype.DataType.UserTypeInstream
import io.kaitai.struct.format._
import io.kaitai.struct.formats.JavaKSYParser
import io.kaitai.struct.testtranslator.specgenerators._
import io.kaitai.struct.{ClassTypeProvider, CppRuntimeConfig}

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

  def getSG(lang: String, testSpec: TestSpec, provider: ClassTypeProvider): BaseGenerator = lang match {
    case "construct" => new ConstructSG(testSpec, provider)
    case "cpp_stl_98" => new CppStlSG(testSpec, provider, CppRuntimeConfig().copyAsCpp98())
    case "cpp_stl_11" => new CppStlSG(testSpec, provider, CppRuntimeConfig().copyAsCpp11())
    case "csharp" => new CSharpSG(testSpec, provider)
    case "go" => new GoSG(testSpec, provider)
    case "java" => new JavaSG(testSpec, provider)
    case "javascript" => new JavaScriptSG(testSpec, provider)
    case "php" => new PHPSG(testSpec, provider)
    case "python" => new PythonSG(testSpec, provider)
    case "ruby" => new RubySG(testSpec, provider)
    case "rust" => new RustSG(testSpec, provider)
  }

  final val ALL_LANGS = List(
    "construct",
    "cpp_stl_98",
    "cpp_stl_11",
    "csharp",
    //"go",
    "java",
    "javascript",
    "php",
    "python",
    "ruby",
    "rust"
  )

  def doAllSpecs(testName: String): Unit = {
    Console.println(s"doAllSpecs($testName)")

    val testSpec = loadTestSpec(testName)
    val classSpecs = loadClassSpecs(testName)
    val initObj = classSpecs(INIT_OBJ_TYPE)
    val provider = new ClassTypeProvider(classSpecs, initObj)

    ALL_LANGS.foreach((langName) => {
      val sg = getSG(langName, testSpec, provider)
      try {
        sg.run()
        writeFile(s"$outDir/$langName/${sg.fileName(testName)}", sg.results)
      } catch {
        case e => e.printStackTrace(Console.err)
      }
    })
  }

  def doOneSpec(langName: String, testName: String) {
    val testSpec = loadTestSpec(testName)
    val classSpecs = loadClassSpecs(testName)
    val initObj = classSpecs(INIT_OBJ_TYPE)
    val provider = new ClassTypeProvider(classSpecs, initObj)

    val sg = getSG(langName, testSpec, provider)
    sg.run()

    writeFile(outDir + "/" + langName + "/" + sg.fileName(testName), sg.results)
  }

  def doAll(): Unit = {
    val dir = new File(specKsDir)
    dir.list().foreach((fn) =>
      if (fn.endsWith(".kst"))
        doAllSpecs(fn.substring(0, fn.size - 4))
    )
  }

  //doOneSpec("hello_world")
  //doOneSpec("process_xor4_const")
  //doOneSpec("enum_0")
  //doOneSpec("cpp_stl", "switch_manual_int_else")
  //doAllSpecs("io_local_var")
  //doAllSpecs("repeat_eos_bit")

  doAll()
}
