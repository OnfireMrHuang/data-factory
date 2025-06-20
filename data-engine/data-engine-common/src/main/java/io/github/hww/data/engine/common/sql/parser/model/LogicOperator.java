package io.github.hww.data.engine.common.sql.parser.model;

public enum LogicOperator {
    And("and"),
    Or("or");

    private final String logicOperator;

    LogicOperator(String logicOperator) {
        this.logicOperator = logicOperator;
    }

    public static LogicOperator fromValue(String logicOperator) {
        for (LogicOperator value : values()) {
            if (value.logicOperator.equals(logicOperator)) {
                return value;
            }
        }
        return null;
    }
}
