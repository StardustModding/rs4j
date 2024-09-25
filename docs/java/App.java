package main;

import org.stardustmodding.rs4j.docs.example.complex.MyStruct;
import org.stardustmodding.rs4j.docs.example.complex.MyOtherStruct;

public class App {
    public static void main(String[] args) {
        System.loadLibrary("docs_test");

        MyOtherStruct s = new MyOtherStruct();
        MyStruct b = s.getB();

        s.sayOnly("First hello!");
        s.getB().setA("Hello, ");
        s.say("world!");
        s.sayWith(b, "hello!");
        b.setA("Second ");
        s.sayWith(b, "hello!");

        s.free();
    }
}
