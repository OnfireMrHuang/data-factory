package io.github.hww.data.engine.common.debezium;

import io.debezium.engine.ChangeEvent;
import io.debezium.engine.format.Json;

import java.io.IOException;

public class DebeziumEngine {

    ConnectorProperties props;

    public DebeziumEngine(ConnectorProperties props) {
        this.props = props;
    }

    public void start() {
        io.debezium.engine.DebeziumEngine<ChangeEvent<String, String>> engine = io.debezium.engine.DebeziumEngine.create(Json.class)
                .using(props.genDebeziumProperties())
                .notifying(record -> {
                    System.out.println(record);
                }).build();
        engine.run();
    }
}
