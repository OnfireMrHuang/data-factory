package io.github.hww.data.foundation.common.entities.monitor;


import lombok.Builder;
import lombok.Getter;

@Getter
@Builder
public class EnvironmentInfo {
    private String nodeManager;
    private String javaVersion;
}
