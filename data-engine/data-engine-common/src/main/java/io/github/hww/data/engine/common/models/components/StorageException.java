package io.github.hww.data.engine.common.models.components;

import lombok.Getter;

@Getter
public class StorageException extends Exception {
    private final String message;
    private String detail;

    public StorageException(String message) {
        this.message = message;
    }

    public StorageException(String message, String detail) {
        this.message = message;
        this.detail = detail;
    }
}
