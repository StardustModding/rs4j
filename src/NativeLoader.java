package $package$;

import cz.adamh.utils.NativeUtils;

public class NativeLoader {
    private static final String libName = "$library$";

    public static void load() {
        OperatingSystem os = OperatingSystem.detect();
        Arch arch = Arch.detect();
        String triple = "%s-%s".formatted(arch.triplePart(), os.triplePart());
        String libPath = "/%s-%s.%s".formatted(libName, triple, os.libExt());

        try {
            NativeUtils.loadLibraryFromJar(libPath);
        } catch (Exception e) {
            // Don't worry about it.
            throw new RuntimeException(e);
        }
    }

    private static enum OperatingSystem {
        MAC,
        LINUX,
        SOLARIS,
        WINDOWS,
        FREEBSD;

        public static OperatingSystem detect() {
            String os = System.getProperty("os.name").toLowerCase();

            if (os.contains("win")) return WINDOWS;
            if (os.contains("mac")) return MAC;
            if (os.contains("linux")) return LINUX;
            if (os.contains("sun")) return SOLARIS;
            if (os.contains("free")) return FREEBSD;

            throw new IllegalArgumentException("Unknown operating system: " + os);
        }

        public String triplePart() {
            switch (this) {
                case MAC:
                    return "apple-darwin";
                case LINUX:
                    return "unknown-linux-gnu";
                case SOLARIS:
                    return "sun-solaris";
                case WINDOWS:
                    return "pc-windows-gnu";
                case FREEBSD:
                    return "unknown-freebsd";
            }

            throw new RuntimeException("How did we get here?");
        }

        public String libExt() {
            switch (this) {
                case MAC:
                    return "dylib";
                case LINUX:
                case FREEBSD:
                case SOLARIS:
                    return "so";
                case WINDOWS:
                    return "dll";
            }

            throw new RuntimeException("How did we get here?");
        }
    }

    private static enum Arch {
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

            switch (arch) {
                case "x86_64":
                case "amd64":
                case "ia64":
                case "x64":
                    return X86_64;
                case "x86_32":
                case "x86":
                case "i386":
                case "i686":
                case "i586":
                case "i486":
                case "ia32":
                case "x32":
                    return X86_32;
                case "sparc":
                case "sparc32":
                    return SPARC_32;
                case "sparcv9":
                case "sparc64":
                    return SPARC_64;
                case "arm":
                case "arm32":
                    return ARM_32;
                case "aarch64":
                case "arm64":
                    return ARM_64;
                case "mips":
                case "mips32":
                    return MIPS_32;
                case "mips64":
                    return MIPS_64;
                case "mipsel":
                case "mips32el":
                    return MIPSEL_32;
                case "mips64el":
                    return MIPSEL_64;
                case "ppc":
                case "ppc32":
                    return PPC_32;
                case "ppc64":
                    return PPC_64;
                case "ppcle":
                case "ppc32le":
                    return PPCLE_32;
                case "ppc64le":
                    return PPCLE_64;
                case "riscv":
                case "riscv32":
                    return RISCV_32;
                case "riscv64":
                    return RISCV_64;
            }

            throw new IllegalArgumentException("Unknown architecture: " + arch);
        }

        public String triplePart() {
            switch (this) {
                case X86_64:
                    return "x86_64";
                case X86_32:
                    return "i686";
                case PPC_32:
                    return "powerpc";
                case PPC_64:
                    return "powerpc64";
                case PPCLE_32:
                    throw new UnsupportedOperationException("ppc32le is not supported by Rust!");
                case PPCLE_64:
                    return "powerpc64le";
                case SPARC_64:
                    return "sparc64";
                case SPARC_32:
                    return "sparc";
                case ARM_32:
                    return "arm";
                case ARM_64:
                    return "aarch64";
                case RISCV_32:
                    return "riscv32gc";
                case RISCV_64:
                    return "riscv64gc";
                case MIPS_32:
                    return "mips";
                case MIPS_64:
                    return "mips64";
                case MIPSEL_32:
                    return "mipsel";
                case MIPSEL_64:
                    return "mips64el";
            }

            throw new RuntimeException("How did we get here?");
        }
    }
}
