use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use anyhow::Result;
use convert_case::{Case, Casing};

use crate::parser::{class::ClassExpr, expr::Expr, func::FunctionExpr};

use super::gen::Generator;

pub fn gen_java_code(gen: Generator, exprs: Vec<Expr>, out: PathBuf) -> Result<()> {
    for item in exprs {
        if let Expr::Class(class) = item {
            gen_class_code(&gen, &out, class)?;
        }
    }

    Ok(())
}

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

import java.util.*;{annotations}

public class {name}{generics} {{\n    private long __pointer;\n\n",
        pkg = gen.package,
        name = class.name.ident()?,
        generics = generics,
        annotations = annotations,
    );

    let suppress = "@SuppressWarnings(\"hiding\")";

    for item in *class.stmts.clone() {
        if let Expr::Function(FunctionExpr {
            args,
            is_mut: _,
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
                    .map(|v| v.0.ident().unwrap())
                    .collect::<Vec<_>>()
                    .join(", ")
            } else {
                let g = generics
                    .iter()
                    .map(|v| v.0.ident().unwrap())
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
                .map(|v| v.0.ident().unwrap())
                .collect::<Vec<_>>()
                .join(", ");

            let f_generics = if f_generics.is_empty() {
                String::new()
            } else {
                format!(" <{}>", f_generics,)
            };

            let ret = ret
                .unwrap_or(Expr::Identifier(String::from("void")))
                .ident_java()?;

            let opt = if is_optional && gen.with_annotations {
                "@Nullable\n    "
            } else {
                ""
            };

            let mut java_args = Vec::new();
            let mut java_args_names = Vec::new();

            for (name, ty, _, _, _) in *args {
                java_args.push(format!(
                    "{} {}",
                    ty.ident_java()?,
                    name.ident_strict_java()?
                ));

                java_args_names.push(name.ident_strict_java()?);
            }

            let java_args = java_args.join(", ");
            let java_args_names = java_args_names.join(", ");

            if is_static {
                code.push_str(&format!(
                    "    {suppress}\n    {opt}private static native{generics_s} {ret} jni_{name}({java_args});\n\n",
                    ret = ret,
                    name = name.ident()?,
                    java_args = java_args,
                    generics_s = generics_s,
                    suppress = suppress,
                    opt = opt,
                ));

                code.push_str(&format!(
                    "    {suppress}\n    {opt}public static{generics_s} {ret} {name}({java_args}) {{\n",
                    ret = ret,
                    name = name.ident()?.to_case(Case::Camel),
                    java_args = java_args,
                    generics_s = generics_s,
                    suppress = suppress,
                    opt = opt,
                ));

                if ret == "void" {
                    code.push_str(&format!(
                        "        {cname}.jni_{name}({java_args_names});\n    }}\n\n",
                        name = name.ident()?,
                        cname = class.name.ident()?,
                        java_args_names = java_args_names,
                    ));
                } else {
                    code.push_str(&format!(
                        "        return {cname}.jni_{name}({java_args_names});\n    }}\n\n",
                        name = name.ident()?,
                        cname = class.name.ident()?,
                        java_args_names = java_args_names,
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
                    "    {suppress}\n    {opt}private native{generics_s} {ret} jni_{name}(long pointer{java_args});\n\n",
                    ret = ret,
                    name = name.ident()?,
                    java_args = java_args_native,
                    generics_s = generics_s,
                    suppress = suppress,
                    opt = opt,
                ));

                code.push_str(&format!(
                    "    {suppress}\n    {opt}public{f_generics} {ret} {name}({java_args}) {{\n",
                    ret = ret,
                    name = name.ident()?.to_case(Case::Camel),
                    java_args = java_args,
                    suppress = suppress,
                    opt = opt,
                ));

                if ret == "void" {
                    code.push_str(&format!(
                        "        this.jni_{name}(this.__pointer{java_args_names});\n    }}\n\n",
                        name = name.ident()?,
                        java_args_names = java_args_names,
                    ));
                } else {
                    code.push_str(&format!(
                        "        return this.jni_{name}(this.__pointer{java_args_names});\n    }}\n\n",
                        name = name.ident()?,
                        java_args_names = java_args_names,
                    ));
                }
            }
        }
    }

    code.push_str("}\n");

    let dir = out.join(gen.dir_pkg());
    let path = dir.join(format!("{}.java", class.name.ident()?));

    if !dir.exists() {
        create_dir_all(dir)?;
    }

    let mut file = File::create(path)?;

    file.write_all(code.as_bytes())?;

    Ok(())
}
