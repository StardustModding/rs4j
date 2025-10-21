package $package$;

import cz.adamh.utils.NativeUtils;

public class NativeLoader {
    private static final String libName = "$library$";

    public static void load() {
        Arch arch = Arch.detect();
        OperatingSystem os = OperatingSystem.detect();
        String triple = "%s-%s".formatted(arch.triplePart(), os.triplePart(arch));
        String libPath = "/%s-%s.%s".formatted(libName, triple, os.libExt());

        try {
            NativeUtils.loadLibraryFromJar(libPath);
        } catch (Exception e) {
            // Don't worry about it.
            throw new RuntimeException(e);
        }
    }

    private enum OperatingSystem {
        MAC, LINUX, SOLARIS, WINDOWS, FREEBSD;

        public static OperatingSystem detect() {
            String os = System.getProperty("os.name").toLowerCase();

            if (os.contains("win")) return WINDOWS;
            if (os.contains("mac")) return MAC;
            if (os.contains("linux")) return LINUX;
            if (os.contains("sun")) return SOLARIS;
            if (os.contains("free")) return FREEBSD;

            throw new IllegalArgumentException("Unknown operating system: " + os);
        }

        public String triplePart(Arch arch) {
            return switch (this) {
                case MAC -> "apple-darwin";
                case LINUX -> arch == Arch.ARM_32 ? "unknown-linux-gnueabihf" : "unknown-linux-gnu";
                case SOLARIS -> "sun-solaris";
                case WINDOWS -> "pc-windows-gnu";
                case FREEBSD -> "unknown-freebsd";
            };
        }

        public String libExt() {
            return switch (this) {
                case MAC -> "dylib";
                case LINUX, FREEBSD, SOLARIS -> "so";
                case WINDOWS -> "dll";
            };
        }
    }

    private enum Arch {
        X86_64,
        X86_32,
        PPC_32,
        PPC_64,
        PPCLE_32,
        PPCLE_64,
        SPARC_64,
        SPARC_32,
        ARM_32,
        ARM_64,
        RISCV_32,
        RISCV_64,
        MIPS_32,
        MIPS_64,
        MIPSEL_32,
        MIPSEL_64;

        public static Arch detect() {
            String arch = System.getProperty("os.arch").toLowerCase();

            return switch (arch) {
                case "x86_64", "amd64", "ia64", "x64" -> X86_64;
                case "x86_32", "x86", "i386", "i686", "i586", "i486", "ia32", "x32" -> X86_32;
                case "sparc", "sparc32" -> SPARC_32;
                case "sparcv9", "sparc64" -> SPARC_64;
                case "arm", "arm32" -> ARM_32;
                case "aarch64", "arm64" -> ARM_64;
                case "mips", "mips32" -> MIPS_32;
                case "mips64" -> MIPS_64;
                case "mipsel", "mips32el" -> MIPSEL_32;
                case "mips64el" -> MIPSEL_64;
                case "ppc", "ppc32" -> PPC_32;
                case "ppc64" -> PPC_64;
                case "ppcle", "ppc32le" -> PPCLE_32;
                case "ppc64le" -> PPCLE_64;
                case "riscv", "riscv32" -> RISCV_32;
                case "riscv64" -> RISCV_64;
                default -> throw new IllegalArgumentException("Unknown architecture: " + arch);
            };
        }

        public String triplePart() {
            return switch (this) {
                case X86_64 -> "x86_64";
                case X86_32 -> "i686";
                case PPC_32 -> "powerpc";
                case PPC_64 -> "powerpc64";
                case PPCLE_32 -> throw new UnsupportedOperationException("ppc32le is not supported by Rust!");
                case PPCLE_64 -> "powerpc64le";
                case SPARC_64 -> "sparc64";
                case SPARC_32 -> "sparc";
                case ARM_32 -> "arm";
                case ARM_64 -> "aarch64";
                case RISCV_32 -> "riscv32gc";
                case RISCV_64 -> "riscv64gc";
                case MIPS_32 -> "mips";
                case MIPS_64 -> "mips64";
                case MIPSEL_32 -> "mipsel";
                case MIPSEL_64 -> "mips64el";
            };
        }
    }
}
