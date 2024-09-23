package com.example;

import java.util.*;

@SuppressWarnings("hiding")
public class HelloWorld {
    private long __pointer;

    private static native String jni_hello();

    public static String hello() {
        return HelloWorld.jni_hello();
    }

}
