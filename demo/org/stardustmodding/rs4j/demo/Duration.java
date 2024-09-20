package org.stardustmodding.rs4j.demo;

import java.util.*;

public class Duration {
    private long __pointer;

    @SuppressWarnings("hiding")
    private static native Duration jni_create(Long secs, Integer nanos);

    @SuppressWarnings("hiding")
    public static Duration create(Long secs, Integer nanos) {
        return Duration.jni_create(secs, nanos);
    }

    @SuppressWarnings("hiding")
    private native Long jni_as_secs(long pointer);

    @SuppressWarnings("hiding")
    public Long asSecs() {
        return this.jni_as_secs(this.__pointer);
    }

}
