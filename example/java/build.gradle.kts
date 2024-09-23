plugins {
    id("application")
}

repositories {
    mavenCentral()
}

dependencies {}

sourceSets.main {
    java {
        srcDir(file("src/generated/java"))
    }

    resources {
        srcDir(file("src/generated/resources"))
    }
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
