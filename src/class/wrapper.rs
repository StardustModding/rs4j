//! Wrapper methods.

use std::collections::BTreeMap;

use convert_case::{Case, Casing};

use super::{ctx::ClassCtx, method::Method, ty::TypeKind};
use crate::codegen::java::{
    JCall, JCtor, JExpr, JMember, JMethodImpl, JSafeFieldCall, JSetField, JType, JVar,
};

impl Method {
    /// Generate Java code for this method.
    pub fn wrapper_java_code(&self, cx: &ClassCtx) -> JMember {
        let native = &self.calls();
        let name = &self.name;
        let class = &cx.name;
        let convert = self.ret.convert_func();
        let mut args = Vec::new();
        let mut args_exprs = Vec::new();

        if !self.is_static {
            args_exprs.push(JExpr::Name("__ptr".into()));
        }

        for arg in &self.args {
            args.push((arg.name.clone(), arg.ty.j_type()));
            args_exprs.push(arg.java_name());
        }

        if self.is_init {
            return JMember::Ctor(JCtor {
                name: class.clone(),
                args,
                private: false,
                code: vec![JExpr::SetField(JSetField {
                    target: "__ptr".into(),
                    value: Box::new(JExpr::Call(JCall {
                        target: format!("jni_init_{name}"),
                        args: args_exprs,
                    })),
                })],
            });
        }

        let c_name = name.to_case(Case::Camel);

        if self.is_mut {
            if self.ret.kind.is_primitive() {
                JMember::MethodImpl(JMethodImpl {
                    args,
                    generics: BTreeMap::new(),
                    is_override: false,
                    is_static: self.is_static,
                    name: c_name,
                    private: false,
                    ret: self.ret.j_type(),

                    code: if self.ret.kind == TypeKind::Void {
                        vec![
                            JExpr::Call(JCall {
                                target: native.clone(),
                                args: args_exprs,
                            }),
                            JExpr::SafeFieldCall(JSafeFieldCall {
                                field: "__parent".into(),
                                target: "updateField".into(),
                                args: vec![
                                    JExpr::Name("__parentField".into()),
                                    JExpr::Name("__ptr".into()),
                                ],
                            }),
                        ]
                    } else {
                        vec![
                            JExpr::Var(JVar {
                                mutable: false,
                                name: "val".into(),
                                ty: self.ret.j_type(),
                                value: Box::new(JExpr::Call(JCall {
                                    target: native.clone(),
                                    args: args_exprs,
                                })),
                            }),
                            JExpr::SafeFieldCall(JSafeFieldCall {
                                field: "__parent".into(),
                                target: "updateField".into(),
                                args: vec![
                                    JExpr::Name("__parentField".into()),
                                    JExpr::Name("__ptr".into()),
                                ],
                            }),
                            JExpr::Return(Box::new(JExpr::Name("val".into()))),
                        ]
                    },
                })
            } else {
                JMember::MethodImpl(JMethodImpl {
                    args,
                    generics: BTreeMap::new(),
                    is_override: false,
                    is_static: self.is_static,
                    name: c_name,
                    private: false,
                    ret: self.ret.j_type(),

                    code: vec![
                        JExpr::Var(JVar {
                            mutable: false,
                            name: "val".into(),
                            ty: JType::Long,
                            value: Box::new(JExpr::Call(JCall {
                                target: native.clone(),
                                args: args_exprs,
                            })),
                        }),
                        JExpr::SafeFieldCall(JSafeFieldCall {
                            field: "__parent".into(),
                            target: "updateField".into(),
                            args: vec![
                                JExpr::Name("__parentField".into()),
                                JExpr::Name("__ptr".into()),
                            ],
                        }),
                        JExpr::Return(Box::new(JExpr::Call(JCall {
                            target: convert,
                            args: vec![JExpr::Name("val".into())],
                        }))),
                    ],
                })
            }
        } else {
            if self.ret.kind.is_primitive() {
                JMember::MethodImpl(JMethodImpl {
                    args,
                    generics: BTreeMap::new(),
                    is_override: false,
                    is_static: self.is_static,
                    name: c_name,
                    private: false,
                    ret: self.ret.j_type(),

                    code: if self.ret.kind == TypeKind::Void {
                        vec![JExpr::Call(JCall {
                            target: native.clone(),
                            args: args_exprs,
                        })]
                    } else {
                        vec![JExpr::Return(Box::new(JExpr::Call(JCall {
                            target: native.clone(),
                            args: args_exprs,
                        })))]
                    },
                })
            } else {
                JMember::MethodImpl(JMethodImpl {
                    args,
                    generics: BTreeMap::new(),
                    is_override: false,
                    is_static: self.is_static,
                    name: c_name,
                    private: false,
                    ret: self.ret.j_type(),

                    code: vec![JExpr::Return(Box::new(JExpr::Call(JCall {
                        target: convert,
                        args: vec![JExpr::Call(JCall {
                            target: native.clone(),
                            args: args_exprs,
                        })],
                    })))],
                })
            }
        }
    }
}
