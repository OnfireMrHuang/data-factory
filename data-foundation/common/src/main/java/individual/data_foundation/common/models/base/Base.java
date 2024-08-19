package individual.data_foundation.common.models.base;

import cn.hutool.core.date.DateTime;
import com.fasterxml.jackson.databind.PropertyNamingStrategy;
import com.fasterxml.jackson.databind.annotation.JsonNaming;
import lombok.Data;
import org.springframework.format.annotation.DateTimeFormat;


@Data
@JsonNaming(PropertyNamingStrategy.SnakeCaseStrategy.class)
public abstract class Base {
    private String id;
    private String createdBy;
    @DateTimeFormat(
            pattern = "yyyy-MM-dd HH:mm:ss"
    )
    private DateTime createdTime;
    private String updatedBy;
    @DateTimeFormat(
            pattern = "yyyy-MM-dd HH:mm:ss"
    )
    private DateTime updatedTime;
}
