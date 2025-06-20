package io.github.hww.data.engine.common.sql.parser.model.exp;

public enum BooleanOperator {

    Eq("="),
    NotEq("<>"),
    Gt(">"),
    GtEq(">="),
    Lt("<"),
    LtEq("<="),
    Like("like"),
    NotLike("not like"),
    In("in"),
    NotIn("not in"),
    IsNull("is null"),
    IsNotNull("is not null"),
    Exists("exists"),
    NotExists("not exists");

    private final String booleanOperator;

    BooleanOperator(String booleanOperator) {
        this.booleanOperator = booleanOperator;
    }

    public static BooleanOperator fromValue(String booleanOperator) {
        for (BooleanOperator value : values()) {
            if (value.booleanOperator.equals(booleanOperator)) {
                return value;
            }
        }
        return null;
    }
}
