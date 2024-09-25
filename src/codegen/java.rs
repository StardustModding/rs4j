//! Java codegen.

use std::{fs, path::PathBuf};

use anyhow::Result;
use convert_case::{Case, Casing};

use crate::{
    class::JavaClassBuilder,
    java::{NATIVE_TOOLS, NATIVE_UTILS, PARENT_CLASS},
    loader::generate_loader,
    parser::{class::ClassExpr, expr::Expr, func::FunctionExpr, ty::TypeExpr},
};

use super::gen::Generator;

/// Generate the Java code for an entire `.rs4j` file.
pub fn gen_java_code(gen: Generator, exprs: Vec<Expr>, out: PathBuf) -> Result<()> {
    for item in exprs {
        if let Expr::Class(class) = item {
            let build =
                JavaClassBuilder::new(class.name.ident_strict()?, &gen.package).of(class.clone());
            // gen_class_code(&gen, &out, class)?;
            let dir = out.join("java/src").join(gen.dir_pkg());
            let path = dir.join(format!("{}.java", class.name.ident()?));
            let code = build.java_code();

            if !dir.exists() {
                fs::create_dir_all(dir)?;
            }

            fs::write(path, code)?;
        }
    }

    let dir = out.join("java/src").join(gen.dir_pkg());
    let path = dir.join("NativeLoader.java");

    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }

    fs::write(path, generate_loader(gen.package, gen.library))?;

    let dir = out.join("java/src/cz/adamh/utils");
    let path = dir.join("NativeUtils.java");

    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }

    fs::write(path, NATIVE_UTILS)?;

    let dir = out.join("java/src/org/stardustmodding/rs4j/util");
    let path = dir.join("NativeTools.java");

    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    fs::write(path, NATIVE_TOOLS)?;

    let path = dir.join("ParentClass.java");

    fs::write(path, PARENT_CLASS)?;

    Ok(())
}

/// Generate Java code for a [`ClassExpr`].
pub fn gen_class_code(gen: &Generator, out: &PathBuf, class: ClassExpr) -> Result<()> {
    let generics = class.generics();

    let generics = if generics.is_empty() {
        String::new()
    } else {
        format!("<{}>", generics)
    };

    let annotations = if gen.with_annotations {
        "\nimport org.jetbrains.annotations.Nullable;"
    } else {
        ""
    };

    let mut code = format!(
        "package {pkg};

import java.util.*;
import org.stardustmodding.rs4j.util.NativeTools;{annotations}

@SuppressWarnings(\"hiding\")
public class {name}{generics} {{\n    private long __pointer;\n\n",
        pkg = gen.package,
        name = class.name.ident()?,
        generics = generics,
        annotations = annotations,
    );

    for item in *class.stmts.clone() {
        if let Expr::Function(FunctionExpr {
            args,
            is_mut: _,
            is_init: _,
            is_static,
            name,
            ret,
            source: _,
            rust_name: _,
            is_optional,
            is_consumed: _,
            generics,
        }) = item
        {
            let c_generics = class.generics();

            let a_generics = if c_generics.is_empty() {
                generics
                    .iter()
                    .map(|v| v.0.as_rust().unwrap())
                    .collect::<Vec<_>>()
                    .join(", ")
            } else {
                let g = generics
                    .iter()
                    .map(|v| v.0.as_rust().unwrap())
                    .collect::<Vec<_>>()
                    .join(", ");

                if g.is_empty() {
                    format!("<{}>", c_generics)
                } else {
                    format!("<{}, {}>", c_generics, g)
                }
            };

            let generics_s = if a_generics.is_empty() {
                String::new()
            } else {
                format!(" {}", a_generics)
            };

            let f_generics = generics
                .iter()
                .map(|v| v.0.as_rust().unwrap())
                .collect::<Vec<_>>()
                .join(", ");

            let f_generics = if f_generics.is_empty() {
                String::new()
            } else {
                format!(" <{}>", f_generics,)
            };

            let ret = ret.unwrap_or(TypeExpr::default());
            let ret_t = ret.as_java()?;

            let opt = if is_optional && gen.with_annotations {
                "@Nullable\n    "
            } else {
                ""
            };

            let mut java_args = Vec::new();
            let mut java_args_names = Vec::new();

            for (name, ty, _, _, _) in *args {
                java_args.push(format!("{} {}", ty.as_java()?, name.ident_strict_java()?));

                java_args_names.push(name.ident_strict_java()?);
            }

            let java_args = java_args.join(", ");
            let java_args_names = java_args_names.join(", ");

            if is_static {
                code.push_str(&format!(
                    "    {opt}private static native{generics_s} long jni_{name}({java_args});\n\n",
                    name = name.ident()?,
                ));

                code.push_str(&format!(
                    "    {opt}public static{generics_s} {ret_t} {name}({java_args}) {{\n",
                    name = name.ident()?.to_case(Case::Camel),
                ));

                if ret_t == "void" {
                    code.push_str(&format!(
                        "        {cname}.jni_{name}({java_args_names});\n    }}\n\n",
                        name = name.ident()?,
                        cname = class.name.ident()?,
                    ));
                } else {
                    code.push_str(&format!(
                        "        long val = {cname}.jni_{name}({java_args_names});\n",
                        name = name.ident()?,
                        cname = class.name.ident()?,
                    ));

                    code.push_str(&format!(
                        "        return {}(val);\n    }}\n\n",
                        ret.conv_method()?,
                    ));
                }
            } else {
                let java_args_names = if java_args_names.len() > 0 {
                    format!(", {}", java_args_names)
                } else {
                    java_args_names
                };

                let java_args_native = if java_args.len() > 0 {
                    format!(", {}", java_args)
                } else {
                    java_args.clone()
                };

                code.push_str(&format!(
                    "    {opt}private native{generics_s} long jni_{name}(long pointer{java_args});\n\n",
                    name = name.ident()?,
                    java_args = java_args_native,
                ));

                code.push_str(&format!(
                    "    {opt}public{f_generics} {ret_t} {name}({java_args}) {{\n",
                    name = name.ident()?.to_case(Case::Camel),
                ));

                if ret_t == "void" {
                    code.push_str(&format!(
                        "        this.jni_{name}(this.__pointer{java_args_names});\n    }}\n\n",
                        name = name.ident()?,
                    ));
                } else {
                    code.push_str(&format!(
                        "        long val = this.jni_{name}(this.__pointer{java_args_names});\n",
                        name = name.ident()?,
                    ));

                    code.push_str(&format!(
                        "        return {}(val);\n    }}\n\n",
                        ret.conv_method()?,
                    ));
                }
            }
        }
    }

    code.push_str("}\n");

    let dir = out.join("java/src").join(gen.dir_pkg());
    let path = dir.join(format!("{}.java", class.name.ident()?));

    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }

    fs::write(path, code)?;

    Ok(())
}
