package io.github.hww.data.engine.common.env;

import lombok.AllArgsConstructor;
import lombok.Getter;

import java.util.Objects;

/**
 * 环境配置枚举
 */
@AllArgsConstructor
@Getter
public enum EnvironmentEnum {

    Data_X_Home("datax.home", "home/app/datax", "dataX安装目录"),
    Data_x_File_Encoding("file.encoding", "UTF-8", "DataX文件编码"),
    Data_x_Logback_Status_Listener_Class("logback.statusListenerClass", "ch.qos.logback.core.status.NopStatusListener", "DataX日志监听器"),
    Data_x_Java_Security_Eg("java.security.egd", "file:///dev/urandom", "DataXJava安全egd"),
    Data_x_Logback_Configuration_File("logback.configurationFile", "/home/app/datax/conf/logback.xml", "DataX日志配置文件");

    /**
     * 环境名称
     */
    private final String propertyName;

    /**
     * 默认值
     */
    private final String defaultValue;

    /**
     * 描述
     */
    private final String description;

    public EnvironmentEnum getForName(String name) {
        for (EnvironmentEnum value : EnvironmentEnum.values()) {
            if (Objects.equals(value.propertyName, name)) {
                return value;
            }
        }
        return null;
    }
}
