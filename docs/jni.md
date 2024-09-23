# Methods

Static signature:

```rs
fn static_method<'local>(mut env: JNIEnv<'local>, class: JClass<'local>, ...args);
```

Instance signature:

```rs
fn instance_method<'local>(mut env: JNIEnv<'local>, this: jobject, ...args);
```
