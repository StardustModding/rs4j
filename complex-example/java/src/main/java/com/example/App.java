package com.example;

public class App {
    static {
        NativeLoader.INSTANCE.load();
    }

    public static void main(String[] args) {
        // The native loader initialization can also be here:
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
        b.getPeople().push("Person 1");
        b.getPeople().push("Person 2");
        b.getPeople().push("Person 3");
        b.getPeople().push("Person 4");
        b.getPeople().push("Person 5");
        b.sayToAll("Hi, everyone!");

        s.free();
    }
}
