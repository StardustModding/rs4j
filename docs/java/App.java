package main;

import org.stardustmodding.rs4j.docs.example.complex.MyOtherStruct;

public class App {
    public static void main(String[] args) {
        System.loadLibrary("docs_test");

        MyOtherStruct s = new MyOtherStruct();

        s.setA("Hello,");

        System.out.println("S: " + s.getA());

        s.setA("world!");

        System.out.println("S: " + s.getA());

        s.getB().setA("2Hello,");

        System.out.println("S: " + s.getB().getA());

        s.getB().setA("2world!");

        System.out.println("S: " + s.getB().getA());

        s.free();
    }
}
