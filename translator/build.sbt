name := "translator"

version := "0.12-SNAPSHOT"

scalaVersion := "2.13.18"

libraryDependencies ++= Seq(
  "io.kaitai" %% "kaitai-struct-compiler" % "0.12-SNAPSHOT",
  "com.github.scopt" %% "scopt" % "4.1.0"
)

mainClass := Some("io.kaitai.struct.testtranslator.Main")
