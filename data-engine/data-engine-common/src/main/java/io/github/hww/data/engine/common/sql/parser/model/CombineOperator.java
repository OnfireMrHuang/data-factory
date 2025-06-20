package io.github.hww.data.engine.common.sql.parser.model;

public enum CombineOperator {
    Union("union"),
    UnionAll("union all"),
    Intersect("intersect"),
    IntersectAll("intersect all"),
    Except("except"),
    ExceptAll("except all");

    private String combineOperator;

    CombineOperator(String combineOperator) {
        this.combineOperator = combineOperator;
    }

    public static CombineOperator fromValue(String combineOperator) {
        for (CombineOperator value : CombineOperator.values()) {
            if (value.combineOperator.equals(combineOperator)) {
                return value;
            }
        }
        return null;
    }
}
