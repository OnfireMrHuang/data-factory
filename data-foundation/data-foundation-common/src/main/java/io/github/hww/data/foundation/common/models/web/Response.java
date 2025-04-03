package io.github.hww.data.foundation.common.models.web;

import com.fasterxml.jackson.annotation.JsonIgnore;
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;
import lombok.Getter;
import org.apache.commons.lang3.ArrayUtils;

import java.io.Serializable;
import java.util.Arrays;
import java.util.Objects;

@Getter
@ApiModel("返回结果")
public class Response<T> implements Serializable {
    @ApiModelProperty(
            value = "是否成功",
            example = "false"
    )
    private Boolean result;
    @ApiModelProperty(
            value = "结果数据",
            example = "1"
    )
    private T data;
    @ApiModelProperty(
            value = "结果码",
            example = "400"
    )
    private Integer code;
    @ApiModelProperty(
            value = "信息提示",
            example = "参数异常"
    )
    private String msg;

    @JsonIgnore
    private Object[] params;
    @JsonIgnore
    private IResponseMessage resultMessage;

    Response() {
    }

    public static <T> Response<T> success() {
        return (Response<T>) success((Object)null);
    }

    public static <T> Response<T> success(T t) {
        Response<T> result = new Response();
        return result.success(true).data(t).message(CommonCodeEnum.SUCCESS);
    }

    public static <E extends IResponseMessage, T> Response<T> fail(E e) {
        Response<T> result = new Response();
        return result.success(false).message(e);
    }

    public static <E extends IResponseMessage, T> Response<T> fail(E e, Object... params) {
        Response<T> result = fail(e);
        result.message(e, params);
        return result;
    }

    public Response<T> data(T t) {
        this.data = t;
        return this;
    }

    public Response<T> success(boolean success) {
        this.result = success;
        return this;
    }

    public <E extends IResponseMessage> Response<T> message(E e) {
        this.code = e.code();
        this.msg = e.message();
        this.resultMessage = e;
        return this;
    }

    public <E extends IResponseMessage> Response<T> message(E e, Object... params) {
        this.code = e.code();
        this.params = params;
        this.resultMessage = e;
        if (ArrayUtils.isNotEmpty(params)) {
            try {
                this.msg = String.format(e.message(), params);
            } catch (Exception var4) {
                this.msg = e.message();
            }
        }

        return this;
    }

    public String toString() {
        return "Response{success=" + this.result + ", data=" + this.data + ", code=" + this.code + ", msg='" + this.msg + '\'' + ", params=" + Arrays.toString(this.params) + '}';
    }

    public boolean equals(Object o) {
        if (this == o) {
            return true;
        } else if (!(o instanceof Response)) {
            return false;
        } else {
            Response<?> resultDTO = (Response)o;
            return Objects.equals(this.result, resultDTO.result) && Objects.equals(this.data, resultDTO.data) && Objects.equals(this.code, resultDTO.code) && Objects.equals(this.msg, resultDTO.msg) && Arrays.equals(this.params, resultDTO.params);
        }
    }

    public int hashCode() {
        int result = Objects.hash(new Object[]{this.result, this.data, this.code, this.msg});
        result = 31 * result + Arrays.hashCode(this.params);
        return result;
    }
}
