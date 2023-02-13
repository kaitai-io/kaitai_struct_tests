name := "translator"

version := "0.11-SNAPSHOT"

scalaVersion := "2.12.12"

resolvers += "Sonatype OSS Snapshots" at "https://oss.sonatype.org/content/repositories/snapshots"
resolvers += Resolver.sonatypeRepo("public")

libraryDependencies ++= Seq(
  "io.kaitai" % "kaitai-struct-compiler_2.12" % "0.11-SNAPSHOT",
  "com.github.scopt" % "scopt_2.12" % "3.7.1",
  "org.yaml" % "snakeyaml" % "1.16"
)

mainClass := Some("io.kaitai.struct.testtranslator.Main")
