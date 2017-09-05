name := "translator"

version := "0.8-SNAPSHOT"

scalaVersion := "2.11.11"

resolvers += "Sonatype OSS Snapshots" at "https://oss.sonatype.org/content/repositories/snapshots"

libraryDependencies ++= Seq(
  "io.kaitai" % "kaitai-struct-compiler_2.11" % "0.8-SNAPSHOT",
  "org.yaml" % "snakeyaml" % "1.16"
)

mainClass := Some("io.kaitai.struct.testtranslator.Main")