package io.github.hww.data.engine.common.components.debezium;

import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Component;
import io.debezium.config.CommonConnectorConfig;
import io.debezium.config.Instantiator;
import io.debezium.engine.ChangeEvent;
import io.debezium.engine.DebeziumEngine;
import io.debezium.engine.format.Json;

import java.util.Properties;

@Slf4j
@Component
public class DebeziumContainer {

    protected DebeziumEngine<ChangeEvent<String, String>> engine;

    private Properties properties = new Properties();

    public void start( DebeziumEngine.ChangeConsumer<ChangeEvent<String, String>> changeConsumer) {
        engine = DebeziumEngine.create(Json.class)
                .using(properties)
                .notifying(changeConsumer).build();
        engine.run();
    }
}
