package io.github.hww.data.engine.common.config.properties;

import lombok.Data;
import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.context.annotation.Configuration;

@Data
@Configuration
@ConfigurationProperties(prefix = "flink")
public class FlinkProperties {
    private String host;
    private String port;
}
