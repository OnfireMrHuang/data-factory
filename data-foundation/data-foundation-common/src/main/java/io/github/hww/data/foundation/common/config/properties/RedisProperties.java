package io.github.hww.data.foundation.common.config.properties;

import lombok.Data;
import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.stereotype.Component;

@Data
@Component
@ConfigurationProperties(prefix = "redis")
public class RedisProperties {
    private String host;
    private String port;
    private String password;
    private String dbIndex;
}
