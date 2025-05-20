package io.github.hww.data.engine.common.models.web;

public interface IResponseMessage {
    Integer code();

    String message();

    default String i18nKey() {
        return "result.message." + this.code();
    }
}
