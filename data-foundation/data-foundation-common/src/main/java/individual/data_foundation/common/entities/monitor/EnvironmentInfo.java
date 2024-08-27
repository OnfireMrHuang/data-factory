package individual.data_foundation.common.entities.monitor;


import lombok.Builder;
import lombok.Getter;

@Getter
@Builder
public class EnvironmentInfo {
    private String nodeManager;
    private String javaVersion;
}
