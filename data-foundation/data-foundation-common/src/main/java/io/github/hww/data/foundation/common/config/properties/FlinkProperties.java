package io.github.hww.data.foundation.common.config.properties;

import lombok.Data;
import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.stereotype.Component;

@Data
@Component
@ConfigurationProperties(prefix = "flink")
public class FlinkProperties {
    private String host;
    private String port;
}
