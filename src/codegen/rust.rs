use crate::parser::{class::ClassExpr, expr::Expr, func::FunctionExpr};
use anyhow::Result;

use super::gen::Generator;

pub fn gen_function(
    gen: &Generator,
    class: &ClassExpr,
    bounds: &String,
    FunctionExpr {
        is_mut,
        is_static,
        name,
        source,
        args: fn_args,
        ret: _,
        rust_name,
        is_optional: _,
        is_consumed,
        generics: _todo,
    }: FunctionExpr,
) -> Result<String> {
    let pkg = gen.jni_pkg();
    let fn_name_id = name.ident()?;
    let fn_name = format!("Java_{}_{}_jni_{}", pkg, class.name.ident()?, fn_name_id);
    let raw_cname = format!("{}.{}", gen.package, class.name.ident()?).replace(".", "/");
    let rust_fn_name = rust_name.unwrap_or(Expr::Identifier(fn_name_id)).ident()?;
    let src = source.unwrap_or(class.real_name.clone().0).ident()?;
    let cname = class.ident_rust()?;
    // let ret = ret.unwrap_or(Expr::Identifier("()".to_string())).ident()?;

    let mut args = Vec::new();
    let mut args_names = Vec::new();

    for (name, ty, borrow, borrow_mut, into) in *fn_args {
        let borrow = if borrow_mut {
            "&mut"
        } else if borrow {
            "&"
        } else {
            ""
        };

        args.push(format!(
            "{}: {}{}",
            name.ident_strict()?,
            borrow,
            ty.ident()?
        ));

        if into {
            args_names.push(format!("{}.into()", name.ident_strict()?));
        } else {
            args_names.push(name.ident_strict()?);
        }
    }

    let args = args.join(",\n    ");
    let args_names = args_names.join(", ");

    let args = if args.len() > 0 {
        format!(",\n    {}", args)
    } else {
        args
    };

    let args_names = if args_names.len() > 0 && !is_static {
        format!(", {}", args_names)
    } else {
        args_names
    };

    let generics = class.generics();
    let generics = if generics.is_empty() {
        String::new()
    } else {
        format!(", {}", generics)
    };

    let function_head = "#[no_mangle]
#[allow(
    unused_mut,
    unused_variables,
    unused_unsafe,
    non_snake_case,
    improper_ctypes_definitions,
    no_mangle_generic_items,
    deprecated,
)]";

    let code = if is_static {
        format!(
            "{function_head}
pub unsafe extern \"system\" fn {fn_name}<'local{generics}>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>{args}
) -> jobject{bounds} {{
    object_to_jobject(env, {src}::{rust_fn_name}({args_names}), \"{raw_cname}\".to_string())
}}",
            fn_name = fn_name,
            generics = generics,
            bounds = bounds,
            rust_fn_name = rust_fn_name,
            function_head = function_head,
            src = src,
            args = args,
            args_names = args_names,
            raw_cname = raw_cname,
        )
    } else {
        if is_mut {
            format!(
                "{function_head}
pub unsafe extern \"system\" fn {fn_name}<'local{generics}>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong{args}
) -> jobject{bounds} {{
    let this: &mut {cname} = jlong_to_pointer::<{cname}>(this).as_mut().unwrap();
    object_to_jobject(env, {src}::{rust_fn_name}(this{args_names}), \"{raw_cname}\".to_string())
}}",
                fn_name = fn_name,
                cname = cname,
                generics = generics,
                bounds = bounds,
                rust_fn_name = rust_fn_name,
                function_head = function_head,
                src = src,
                args = args,
                args_names = args_names,
                raw_cname = raw_cname,
            )
        } else if is_consumed {
            format!(
                "{function_head}
pub unsafe extern \"system\" fn {fn_name}<'local{generics}>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong{args}
) -> jobject{bounds} {{
    let this: &{cname} = jlong_to_pointer::<{cname}>(this).as_mut().unwrap();
    let this = this.clone();
    object_to_jobject(env, {src}::{rust_fn_name}(this{args_names}), \"{raw_cname}\".to_string())
}}",
                fn_name = fn_name,
                cname = cname,
                generics = generics,
                bounds = bounds,
                rust_fn_name = rust_fn_name,
                function_head = function_head,
                src = src,
                args = args,
                args_names = args_names,
                raw_cname = raw_cname,
            )
        } else {
            format!(
                "{function_head}
pub unsafe extern \"system\" fn {fn_name}<'local{generics}>(
    mut env: *mut jni::JNIEnv<'local>,
    class: jni::objects::JClass<'local>,
    this: jlong{args}
) -> jobject{bounds} {{
    let this: &{cname} = jlong_to_pointer::<{cname}>(this).as_mut().unwrap();
    object_to_jobject(env, {src}::{rust_fn_name}(this{args_names}), \"{raw_cname}\".to_string())
}}",
                fn_name = fn_name,
                cname = cname,
                generics = generics,
                bounds = bounds,
                rust_fn_name = rust_fn_name,
                function_head = function_head,
                src = src,
                args = args,
                args_names = args_names,
                raw_cname = raw_cname,
            )
        }
    };

    Ok(code)
}
