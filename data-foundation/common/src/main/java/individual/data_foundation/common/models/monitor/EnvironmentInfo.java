package individual.data_foundation.common.models.monitor;


import lombok.Builder;
import lombok.Getter;

@Getter
@Builder
public class EnvironmentInfo {
    private String nodeManager;
    private String javaVersion;
}
