package $package$

import cz.adamh.utils.NativeUtils.loadLibraryFromJar
import java.util.*

object NativeLoader {
    private const val LIB_NAME = "$library$"

    fun load() {
        val arch = Arch.detect()
        val os = OperatingSystem.detect()
        val triple = "${arch.triplePart}-${os.triplePart(arch)}"
        val libPath = "/$LIB_NAME-$triple.${os.libExt}"

        try {
            loadLibraryFromJar(libPath)
        } catch (e: Exception) {
            // Don't worry about it.
            throw RuntimeException(e)
        }
    }

    private enum class OperatingSystem {
        MAC, LINUX, SOLARIS, WINDOWS, FREEBSD;

        fun triplePart(arch: Arch) = when (this) {
            MAC -> "apple-darwin"
            LINUX -> if (arch == Arch.ARM_32) "unknown-linux-gnueabihf" else "unknown-linux-gnu"
            SOLARIS -> "sun-solaris"
            WINDOWS -> "pc-windows-gnu"
            FREEBSD -> "unknown-freebsd"
        }

        val libExt
            get() = when (this) {
                MAC -> "dylib"
                LINUX, FREEBSD, SOLARIS -> "so"
                WINDOWS -> "dll"
            }

        companion object {
            fun detect(): OperatingSystem {
                val os = System.getProperty("os.name").lowercase(Locale.getDefault())

                if (os.contains("win")) return WINDOWS
                if (os.contains("mac")) return MAC
                if (os.contains("linux")) return LINUX
                if (os.contains("sun")) return SOLARIS
                if (os.contains("free")) return FREEBSD

                throw IllegalArgumentException("Unknown operating system: $os")
            }
        }
    }

    private enum class Arch {
        X86_64, X86_32, PPC_32, PPC_64, PPCLE_32, PPCLE_64, SPARC_64, SPARC_32, ARM_32, ARM_64, RISCV_32, RISCV_64, MIPS_32, MIPS_64, MIPSEL_32, MIPSEL_64;

        val triplePart
            get() = when (this) {
                X86_64 -> "x86_64"
                X86_32 -> "i686"
                PPC_32 -> "powerpc"
                PPC_64 -> "powerpc64"
                PPCLE_32 -> throw UnsupportedOperationException("ppc32le is not supported by Rust!")
                PPCLE_64 -> "powerpc64le"
                SPARC_64 -> "sparc64"
                SPARC_32 -> "sparc"
                ARM_32 -> "arm"
                ARM_64 -> "aarch64"
                RISCV_32 -> "riscv32gc"
                RISCV_64 -> "riscv64gc"
                MIPS_32 -> "mips"
                MIPS_64 -> "mips64"
                MIPSEL_32 -> "mipsel"
                MIPSEL_64 -> "mips64el"
            }

        companion object {
            fun detect() = when (val arch = System.getProperty("os.arch").lowercase(Locale.getDefault())) {
                "x86_64", "amd64", "ia64", "x64" -> X86_64
                "x86_32", "x86", "i386", "i686", "i586", "i486", "ia32", "x32" -> X86_32
                "sparc", "sparc32" -> SPARC_32
                "sparcv9", "sparc64" -> SPARC_64
                "arm", "arm32" -> ARM_32
                "aarch64", "arm64" -> ARM_64
                "mips", "mips32" -> MIPS_32
                "mips64" -> MIPS_64
                "mipsel", "mips32el" -> MIPSEL_32
                "mips64el" -> MIPSEL_64
                "ppc", "ppc32" -> PPC_32
                "ppc64" -> PPC_64
                "ppcle", "ppc32le" -> PPCLE_32
                "ppc64le" -> PPCLE_64
                "riscv", "riscv32" -> RISCV_32
                "riscv64" -> RISCV_64
                else -> throw IllegalArgumentException("Unknown architecture: $arch")
            }
        }
    }
}
