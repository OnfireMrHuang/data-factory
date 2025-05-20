package io.github.hww.data.engine.common.entities.monitor;


import lombok.Builder;
import lombok.Getter;

@Getter
@Builder
public class EnvironmentInfo {
    private String nodeManager;
    private String javaVersion;
}
