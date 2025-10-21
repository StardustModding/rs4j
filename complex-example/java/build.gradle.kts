plugins {
    id("application")
    kotlin("jvm") version "2.1.21"
}

repositories {
    mavenCentral()
}

dependencies { implementation(kotlin("stdlib-jdk8")) }

sourceSets.main {
    java.srcDir(file("src/generated/java"))
    kotlin.srcDir(file("src/generated/kotlin"))
    resources.srcDir(file("src/generated/resources"))
}

java {
    toolchain {
        languageVersion = JavaLanguageVersion.of(21)
    }
}

application {
    mainClass = "com.example.App"
}

tasks.jar {
    manifest {
        attributes(mapOf(
            "Main-Class" to "com.example.App",
        ))
    }
}
