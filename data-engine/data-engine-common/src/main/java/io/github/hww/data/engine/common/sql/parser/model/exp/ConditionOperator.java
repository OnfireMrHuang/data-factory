package io.github.hww.data.engine.common.sql.parser.model.exp;

public enum ConditionOperator {

    And("and"),
    Or("or");

    private final String conditionOperator;

    ConditionOperator(String conditionOperator) {
        this.conditionOperator = conditionOperator;
    }

    public static ConditionOperator fromValue(String conditionOperator) {
        for (ConditionOperator value : values()) {
            if (value.conditionOperator.equals(conditionOperator)) {
                return value;
            }
        }
        return null;
    }
}
