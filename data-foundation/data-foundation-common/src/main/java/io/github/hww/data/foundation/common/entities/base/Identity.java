package io.github.hww.data.foundation.common.entities.base;

import cn.hutool.core.date.DateTime;
import lombok.Data;


@Data
public class Identity {
    private String id;
    private DateTime createdTime;
    private DateTime updatedTime;
}
