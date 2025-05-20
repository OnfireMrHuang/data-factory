package io.github.hww.data.foundation.common.models.web;

import lombok.Getter;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Arrays;

@Getter
public class Exception extends RuntimeException implements IResponseMessage {
    private static final Logger log = LoggerFactory.getLogger(Exception.class);
    private Integer code;
    private String message;
    private Object[] params;

    public Exception() {}

    public static <E extends IResponseMessage> Exception wrapThrow(E e) {
        Exception result = new Exception();
        result.code = e.code();
        result.message = e.message();
        return result;
    }

    public static <E extends IResponseMessage> Exception wrapThrow(E e, Object... params) {
        Exception result = new Exception();
        result.code = e.code();
        result.message = e.message();
        result.params = params;
        return result;
    }

    @Override
    public Integer code() {
        return code;
    }

    @Override
    public String message() {
        return message;
    }

    public String toString() {
        return "Exception{code=" + this.code + ", message='" + this.message + '\'' + ", params=" + Arrays.toString(this.params) + "} ";
    }
}
