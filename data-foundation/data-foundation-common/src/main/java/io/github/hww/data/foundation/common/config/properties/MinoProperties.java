package io.github.hww.data.foundation.common.config.properties;

import lombok.Data;
import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.stereotype.Component;

@Data
@Component
@ConfigurationProperties(prefix = "mino")
public class MinoProperties {
    private String bucket;
    private String endpoint;
    private String accessKey;
    private String accessSecret;
    private String region;
}
