//! The module for [`Class`]es.

use base::{free_method_java, free_method_java_wrapper, free_method_rust, of_func};
use ctx::ClassCtx;
use field::Field;
use generic::TypeGeneric;
use method::Method;

use crate::if_else;

pub mod arg;
pub mod base;
pub mod conv;
pub mod ctx;
pub mod expr;
pub mod field;
pub mod generic;
pub mod method;
pub mod native;
pub mod ty;
pub mod wrapper;

/// A Java class.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Class {
    /// The name of this class.
    pub name: String,

    /// The package
    pub package: String,

    /// A list of imports.
    pub imports: Vec<String>,

    /// A list of fields.
    pub fields: Vec<Field>,

    /// A list of methods.
    pub methods: Vec<Method>,

    /// A list of generics.
    pub generics: Vec<TypeGeneric>,

    /// Should it be a wrapper?
    pub wrapped: bool,
}

impl Class {
    /// Create a new [`Class`].
    pub fn new(name: impl AsRef<str>, package: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().into(),
            package: package.as_ref().into(),
            imports: Self::default_imports(),
            fields: Vec::new(),
            methods: Vec::new(),
            generics: Vec::new(),
            wrapped: false,
        }
    }

    /// Get default imports.
    pub fn default_imports() -> Vec<String> {
        vec![
            "import java.util.*;".into(),
            "import org.stardustmodding.rs4j.util.NativeTools;".into(),
            "import org.stardustmodding.rs4j.util.ParentClass;".into(),
            "import org.stardustmodding.rs4j.util.NativeClass;".into(),
        ]
    }

    /// Set the package name.
    pub fn set_package(mut self, pkg: impl AsRef<str>) -> Self {
        self.package = pkg.as_ref().to_string();
        self
    }

    /// Add an import.
    pub fn import(mut self, import: impl AsRef<str>) -> Self {
        self.imports.push(import.as_ref().to_string());
        self
    }

    /// Add a field.
    pub fn field(mut self, field: Field) -> Self {
        self.fields.push(field);
        self
    }

    /// Add a native method.
    pub fn method(mut self, method: Method) -> Self {
        self.methods.push(method);
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
        let class_g = cx.raw_name_generics_java();
        let class_ge = cx.raw_name_generics();
        let g = cx.generics_java();

        let head = format!(
            "package {pkg};\n\n{imports}\n\npublic class {class_g} implements ParentClass, NativeClass {{\n"
        );

        for func in &self.methods {
            natives.push(func.native_java_code());
        }

        for func in &self.methods {
            wrappers.push(func.wrapper_java_code(&cx));
        }

        for field in &self.fields {
            if field.rust {
                continue;
            }

            let name = &field.name;

            natives.push(field.java_setter());
            natives.push(field.java_getter());

            fields.push(field.java_setter_wrapper());
            fields.push(field.java_getter_wrapper());

            if !field.is_primitive() {
                update_fields.push(format!("        if (field == \"{name}\") {{\n            __ptr = jni_set_{name}(__ptr, pointer);\n        }}"));
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
            "    public static {g} {class_ge} from(long ptr) {{
        return new {class_ge}(ptr);
    }}

    public static {g} {class_ge} from(long ptr, ParentClass parent, String parentField) {{
        return new {class_ge}(ptr, parent, parentField);
    }}"
        );

        let get_ptr = "    @Override\n    public long getPointer() {\n        return __ptr;\n    }";

        let update = format!(
            "    @Override\n    public void updateField(String field, long pointer) {{\n{}\n    }}",
            update_fields.join("\n")
        );

        format!("{head}{natives}\n\n{vars}\n\n{wrappers}\n\n{fields}\n\n{inits}\n\n{froms}\n\n{get_ptr}\n\n{update}\n}}")
    }

    /// Generate rust bindgen code
    pub fn rust_code(&self) -> String {
        let cx = self.new_context();
        let mut code = Vec::new();

        for f in &self.fields {
            if f.rust {
                continue;
            }

            code.push(f.rust_setter(&cx));
            code.push(f.rust_getter(&cx));
        }

        for m in &self.methods {
            code.push(m.native_rust_code(&cx, &self.fields, &self.generics));
        }

        code.push(free_method_rust(&cx, &self.fields));

        format!("{}\n{}", self.create_wrapper(), code.join("\n\n"))
    }

    /// Create the Rust code for a wrapper struct.
    pub fn create_wrapper(&self) -> String {
        let head = "#[allow(
        unused_mut,
        unused_variables,
        unused_unsafe,
        non_snake_case,
        improper_ctypes_definitions,
        no_mangle_generic_items,
        deprecated,
        missing_docs,
    )]";

        let cx = self.new_context();
        let mut fields = Vec::new();

        let generics = self
            .generics
            .iter()
            .map(|v| v.code())
            .collect::<Vec<_>>()
            .join(", ");

        let generics = if_else!(generics != "", format!("<{}>", generics), "".into());

        let generics_nb = self
            .generics
            .iter()
            .map(|v| v.name.to_owned())
            .collect::<Vec<_>>()
            .join(", ");

        let generics_nb = if_else!(generics_nb != "", format!("<{}>", generics_nb), "".into());

        if self.wrapped {
            fields.push(format!("    pub __inner: {},", &cx.raw_name_generics()));
        }

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
            "#[allow(non_camel_case_types)]\npub struct __JNI_{}{} {{\n{}\n}}",
            self.name,
            generics,
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

        impls.push(of_func(&cx, &self.fields));

        if self.wrapped {
            impls.push(format!(
                "    {head}\n    pub unsafe fn to_rust(&self) -> {}{generics_nb} {{\n        self.__inner.clone()\n    }}",
                self.name,
            ));
        } else {
            impls.push(format!(
                "    {head}\n    pub unsafe fn to_rust(&self) -> {}{generics_nb} {{\n        {} {{\n{}\n        }}\n    }}",
                self.name,
                self.name,
                convert.join("\n")
            ));
        }

        for native in &self.methods {
            impls.push(native.native_rust_wrapper_code(&cx));
        }

        let impl_ = format!(
            "impl{} __JNI_{}{} {{\n{}\n}}\n",
            generics,
            self.name,
            generics_nb,
            impls.join("\n\n")
        );

        format!("{}\n\n{}", struct_, impl_)
    }

    /// Create a new [`ClassCtx`]
    fn new_context(&self) -> ClassCtx {
        ClassCtx::new(self)
    }
}
