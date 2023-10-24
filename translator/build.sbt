name := "translator"

version := "0.10-SNAPSHOT"

scalaVersion := "2.12.8"

resolvers += "Sonatype OSS Snapshots" at "https://oss.sonatype.org/content/repositories/snapshots"
resolvers ++= Resolver.sonatypeOssRepos("public")

libraryDependencies ++= Seq(
  "io.kaitai" %% "kaitai-struct-compiler" % "0.11-SNAPSHOT",
  "com.github.scopt" %% "scopt" % "3.7.1",
  "org.yaml" % "snakeyaml" % "1.16"
)

mainClass := Some("io.kaitai.struct.testtranslator.Main")
