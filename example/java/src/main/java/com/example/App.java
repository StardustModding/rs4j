package com.example;

public class App {
    static {
        NativeLoader.load();
    }
    
    public static void main(String[] args) {
        System.out.println(HelloWorld.hello());
    }
}
