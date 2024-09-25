package com.example;

public class App {
    static {
        NativeLoader.load();
    }

    public static void main(String[] args) {
        // It can also be here:
        // NativeLoader.load();
        // It just needs to be called before any native objects are used.
        
        MyOtherStruct s = new MyOtherStruct();
        MyStruct b = s.getB();

        s.sayOnly("First hello!");  // "First hello!"
        s.getB().setA("Hello, ");
        s.say("world!"); // "Hello, world!"
        s.sayWith(b, "hello!"); // "Hello, hello!"
        b.setA("Second ");
        s.sayWith(b, "hello!"); // "Second hello!"

        s.free();
    }
}
