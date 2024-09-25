//! The module for [`JavaClassBuilder`]s.

use base::{free_method_java, free_method_java_wrapper};
use ctx::ClassCtx;
use field::Field;
use native::NativeMethod;
use wrapper::WrapperMethod;

use crate::parser::{class::ClassExpr, expr::Expr, field::FieldExpr, func::FunctionExpr};

pub mod arg;
pub mod base;
pub mod conv;
pub mod ctx;
pub mod field;
pub mod native;
pub mod ty;
pub mod wrapper;

/// A builder for a Java class.
pub struct JavaClassBuilder {
    /// The name of this class.
    pub name: String,

    /// The package
    pub package: String,

    /// A list of imports.
    pub imports: Vec<String>,

    /// A list of fields.
    pub fields: Vec<Field>,

    /// A list of native methods.
    pub native_methods: Vec<NativeMethod>,

    /// A list of wrapper methods.
    pub wrapper_methods: Vec<WrapperMethod>,
}

impl JavaClassBuilder {
    /// Create a new [`JavaClassBuilder`].
    pub fn new(name: impl AsRef<str>, package: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().into(),
            package: package.as_ref().into(),
            imports: vec![
                "import java.util.*;".into(),
                "import org.stardustmodding.rs4j.util.NativeTools;".into(),
                "import org.stardustmodding.rs4j.util.ParentClass;".into(),
            ],
            fields: Vec::new(),
            native_methods: Vec::new(),
            wrapper_methods: Vec::new(),
        }
    }

    /// Add all exprs from a [`ClassExpr`].
    pub fn of(mut self, class: ClassExpr) -> Self {
        for stmt in class.stmts.iter() {
            if let Expr::Function(expr) = stmt {
                self.native_methods.push(expr.clone().into());
                self.wrapper_methods.push(expr.clone().into());
            } else if let Expr::Field(expr) = stmt {
                self.fields.push(expr.clone().into());
            }
        }

        self
    }

    /// Add all [`FunctionExpr`]s.
    pub fn add_func_exprs(mut self, exprs: Vec<FunctionExpr>) -> Self {
        for expr in exprs {
            self.native_methods.push(expr.clone().into());
            self.wrapper_methods.push(expr.clone().into());
        }

        self
    }

    /// Add all [`FieldExpr`]s.
    pub fn add_field_exprs(mut self, exprs: Vec<FieldExpr>) -> Self {
        for expr in exprs {
            self.fields.push(expr.into());
        }

        self
    }

    /// Add an import.
    pub fn import(mut self, import: String) -> Self {
        self.imports.push(import);
        self
    }

    /// Add a field.
    pub fn field(mut self, field: Field) -> Self {
        self.fields.push(field);
        self
    }

    /// Add a native method.
    pub fn native(mut self, method: NativeMethod) -> Self {
        self.native_methods.push(method);
        self
    }

    /// Add a native method.
    pub fn wrapper(mut self, method: WrapperMethod) -> Self {
        self.wrapper_methods.push(method);
        self
    }

    /// Create the Java code.
    pub fn java_code(&self) -> String {
        let pkg = &self.package;
        let imports = self.imports.join("\n");
        let class = &self.name;
        let cx = self.new_context();
        let mut natives = Vec::new();
        let mut wrappers = Vec::new();
        let mut fields = Vec::new();
        let mut update_fields = Vec::new();

        let head = format!(
            "package {pkg};\n\n{imports}\n\npublic class {class} implements ParentClass {{\n"
        );

        for func in &self.native_methods {
            natives.push(func.java_code());
        }

        for func in &self.wrapper_methods {
            wrappers.push(func.java_code(&cx));
        }

        for field in &self.fields {
            let name = &field.name;

            fields.push(field.java_setter());
            fields.push(field.java_getter());

            fields.push(field.java_setter_wrapper());
            fields.push(field.java_getter_wrapper());

            if !field.is_primitive() {
                update_fields.push(format!("        if (field == \"{name}\") {{\n            __ptr = jni_set_{name}(pointer);\n        }}"));
            }
        }

        natives.push(free_method_java().into());
        wrappers.push(free_method_java_wrapper().into());

        let natives = natives.join("\n");
        let wrappers = wrappers.join("\n\n");
        let fields = fields.join("\n\n");

        let vars = "    private long __ptr = -1;\n    private ParentClass __parent = null;\n    private String __parentField = null;";

        let inits = format!(
            "    private {class}(long ptr) {{
        __ptr = ptr;
    }}

    private {class}(long ptr, ParentClass parent, String parentField) {{
        __ptr = ptr;
        __parent = parent;
        __parentField = parentField;
    }}"
        );

        let froms = format!(
            "    public static {class} from(long ptr) {{
        return new {class}(ptr);
    }}

    public static {class} from(long ptr, ParentClass parent, String parentField) {{
        return new {class}(ptr, parent, parentField);
    }}"
        );

        let get_ptr = "    public long getPointer() {\n        return __ptr;\n    }";

        let update = format!(
            "    @Override\n    public void updateField(String field, long pointer) {{\n{}\n    }}",
            update_fields.join("\n")
        );

        format!("{head}{natives}\n{vars}\n{wrappers}\n{fields}\n{inits}\n{froms}\n{get_ptr}\n{update}\n}}")
    }

    /// Generate rust bindgen code
    pub fn rust_code(&self) -> String {
        let cx = self.new_context();
        let mut code = Vec::new();

        for f in &self.fields {
            code.push(f.rust_setter(&cx));
            code.push(f.rust_getter(&cx));
        }

        for m in &self.native_methods {
            code.push(m.rust_code(&cx));
        }

        format!("{}\n{}", self.create_wrapper(), code.join("\n\n"))
    }

    /// Create the Rust code for a wrapper struct.
    pub fn create_wrapper(&self) -> String {
        let mut fields = Vec::new();

        for field in &self.fields {
            if field.is_primitive() {
                fields.push(format!("    pub {}: {},", field.name, field.ty.full_type()));
            } else {
                fields.push(format!(
                    "    pub {}: *mut {},",
                    field.name,
                    field.ty.full_type()
                ));
            }
        }

        let struct_ = format!(
            "#[allow(non_camel_case_types)]\npub struct __JNI_{} {{\n{}\n}}",
            self.name,
            fields.join("\n")
        );

        let mut impls = Vec::new();
        let mut convert = Vec::new();

        for field in &self.fields {
            if field.is_primitive() {
                convert.push(format!(
                    "            {}: self.{}.clone(),",
                    field.name, field.name
                ));
            } else {
                convert.push(format!(
                    "            {}: (&mut *self.{}).clone(),",
                    field.name, field.name
                ));
            }
        }

        impls.push(format!(
            "    pub unsafe fn to_rust(&self) -> {} {{\n        {} {{\n{}\n        }}\n    }}",
            self.name,
            self.name,
            convert.join("\n")
        ));

        let cx = self.new_context();

        for native in &self.native_methods {
            impls.push(native.rust_code_wrapper(&cx));
        }

        let impl_ = format!("impl __JNI_{} {{{}\n}}\n", self.name, impls.join("\n\n"));

        format!("{}\n\n{}", struct_, impl_)
    }

    /// Create a new [`ClassCtx`]
    fn new_context(&self) -> ClassCtx {
        ClassCtx::new(self)
    }
}