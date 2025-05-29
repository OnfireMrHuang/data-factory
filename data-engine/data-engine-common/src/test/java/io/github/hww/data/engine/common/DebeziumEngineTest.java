package io.github.hww.data.engine.common;

import io.github.hww.data.engine.common.debezium.ConnectorProperties;
import io.github.hww.data.engine.common.debezium.DebeziumEngine;
import org.apache.commons.lang3.StringUtils;
import org.junit.Test;

import java.io.File;

public class DebeziumEngineTest {

    @Test
    public void testMysqlToStdio() {
        String password = System.getenv("password");
        if (StringUtils.isEmpty(password)) {
            throw new RuntimeException("password is not exists");
        }
        ConnectorProperties props = ConnectorProperties.builder()
                .name("debezium-mysql-connector")
                .className("io.debezium.connector.mysql.MySqlConnector")
                .offsetStorageClass("org.apache.kafka.connect.storage.FileOffsetBackingStore")
                .offsetStorageFileName("/tmp/offsets.dat")
                .offsetFlushInterval("5000")
                .databaseHost("localhost")
                .databasePort("3306")
                .databaseUser("root")
                .databasePassword(password)
                .databaseServerId("1001")
                .schemaHistoryInternalClass("io.debezium.storage.file.history.FileSchemaHistory")
                .schemaHistoryInternalFileName("/tmp/dbhistory.dat")
                .build();
        DebeziumEngine engine = new DebeziumEngine(props);
        engine.start();
    }
}
