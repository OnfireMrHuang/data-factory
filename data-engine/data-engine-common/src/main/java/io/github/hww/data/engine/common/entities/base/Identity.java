package io.github.hww.data.engine.common.entities.base;

import com.fasterxml.jackson.annotation.JsonFormat;
import lombok.Data;

import java.time.LocalDateTime;


@Data
public class Identity {
    private String id;
    // 定义时间格式
    @JsonFormat(
            pattern = "yyy-MM-dd HH:mm:ss",
            timezone = "GMT+08:00"
    )
    private LocalDateTime createdTime;
    @JsonFormat(
            pattern = "yyy-MM-dd HH:mm:ss",
            timezone = "GMT+08:00"
    )
    private LocalDateTime updatedTime;
}
