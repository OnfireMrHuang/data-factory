package io.github.hww.data.foundation.common.config.properties;

import lombok.Data;
import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.context.annotation.Configuration;

@Data
@Configuration
@ConfigurationProperties(prefix = "mysql")
public class MysqlProperties {
    private String host;
    private String port;
    private String username;
    private String password;
    private String database;
}
