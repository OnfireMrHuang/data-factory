package io.github.hww.data.engine.common.config.properties;

import lombok.Data;
import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.context.annotation.Configuration;

@Data
@Configuration
@ConfigurationProperties(prefix = "kafka")
public class KafkaProperties {
    private String host;
    private String port;
    private String user;
    private String password;
}
