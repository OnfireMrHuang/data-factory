package individual.data_foundation.common.models.base;

import cn.hutool.core.date.DateTime;
import com.fasterxml.jackson.databind.PropertyNamingStrategy;
import com.fasterxml.jackson.databind.annotation.JsonNaming;
import lombok.Data;
import org.springframework.format.annotation.DateTimeFormat;


@Data
@JsonNaming(PropertyNamingStrategy.SnakeCaseStrategy.class)
public abstract class IdentityWithOperator extends Identity {
    private String createdBy;
    private String updatedBy;
}
