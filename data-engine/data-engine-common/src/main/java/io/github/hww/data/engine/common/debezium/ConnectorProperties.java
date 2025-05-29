package io.github.hww.data.engine.common.debezium;

import lombok.Builder;
import lombok.Getter;

import java.util.Properties;

@Builder
@Getter
public class ConnectorProperties {

    private String name; // 名称

    private String className; // Java类名

    private String offsetStorageClass; // 采集位点存储方式类名

    private String offsetStorageFileName; // 采集位点存储文件名

    private String offsetFlushInterval; // 采集位点刷新间隔

    private String databaseHost; // 采集数据库地址

    private String databasePort; // 采集数据库端口

    private String databaseUser; // 采集数据库用户名

    private String databasePassword; // 采集数据库密码

    private String databaseServerId; // 采集数据库实例ID

    private String schemaHistoryInternalClass; // 采集数据库持久化schema历史变更记录的类名

    private String schemaHistoryInternalFileName; // 采集数据库持久化schema历史变更记录的文件名


    public Properties genDebeziumProperties() {
        Properties props = new Properties();
        props.setProperty("name", name);
        props.setProperty("connector.class", className);
        props.setProperty("offset.storage", offsetStorageClass);
        props.setProperty("offset.storage.file.filename", offsetStorageFileName);
        props.setProperty("offset.flush.interval.ms", offsetFlushInterval);
        /* begin connector properties */
        props.setProperty("database.hostname", databaseHost);
        props.setProperty("database.port", databasePort);
        props.setProperty("database.user", databaseUser);
        props.setProperty("database.password", databasePassword);
        props.setProperty("database.server.id", databaseServerId);
        props.setProperty("topic.prefix", "stream-connector");
        props.setProperty("schema.history.internal", schemaHistoryInternalClass);
        props.setProperty("schema.history.internal.file.filename", schemaHistoryInternalFileName);
        return props;
    }
}
