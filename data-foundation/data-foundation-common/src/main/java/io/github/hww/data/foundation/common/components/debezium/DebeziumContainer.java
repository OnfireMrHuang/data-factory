package io.github.hww.data.foundation.common.components.debezium;

import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Component;
import io.debezium.config.CommonConnectorConfig;
import io.debezium.config.Instantiator;
import io.debezium.engine.ChangeEvent;
import io.debezium.engine.DebeziumEngine;
import io.debezium.engine.format.Json;

@Slf4j
@Component
public class DebeziumContainer {

    engine = DebeziumEngine.create(Json.class)
            .using(properties)
                .notifying(changeConsumer).build();
}
