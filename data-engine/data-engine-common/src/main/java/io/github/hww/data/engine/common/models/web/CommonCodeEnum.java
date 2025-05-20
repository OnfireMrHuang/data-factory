package io.github.hww.data.engine.common.models.web;

public enum CommonCodeEnum implements IResponseMessage {
    SUCCESS(200, "成功"),
    INVALID_PARAM(400, "%s"),
    ERROR_SYSTEM(500, "系统繁忙，请稍后再试 \n%s"),
    UNAUTHORIZED_USER(401, "无访问权限");

    private Integer code;
    private String message;

    private CommonCodeEnum(Integer code, String message) {
        this.code = code;
        this.message = message;
    }

    public Integer code() {
        return this.code;
    }

    public String message() {
        return this.message;
    }
}
