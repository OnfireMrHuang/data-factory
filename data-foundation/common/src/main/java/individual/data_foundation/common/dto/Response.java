package individual.data_foundation.common.dto;

import com.fasterxml.jackson.databind.PropertyNamingStrategy;
import com.fasterxml.jackson.databind.annotation.JsonNaming;
import individual.data_foundation.common.exception.IResponse;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;


import java.io.Serializable;

@Data
@Builder
@AllArgsConstructor
@JsonNaming(PropertyNamingStrategy.SnakeCaseStrategy.class)
public class Response<T> implements Serializable {
    private Boolean result;
    private Integer errCode;
    private String errMessage;
    private String userTip;
    private T data;

    public Response() {}

    public static <T> Response<T> success() {
        return success(null);
    }

    public static <T> Response<T> success(T t) {
        return Response.<T>builder().result(true).errCode(200).errMessage("success").data(t).build();
    }

    public static <E extends IResponse> Response<E> fail(E e) {
        return Response.<E>builder().result(false).errCode(e.code()).errMessage(e.message()).build();
    }
}
