package io.github.hww.data.engine.common.sql.parser.model;

public enum JoinTableType {
    InnerJoin("inner join"),
    LeftJoin("left join"),
    RightJoin("right join"),
    FullJoin("full join");

    private String joinType;

    JoinTableType(String joinType) {
        this.joinType = joinType;
    }

    public static JoinTableType fromValue(String joinType) {
        for (JoinTableType value : values()) {
            if (value.joinType.equals(joinType)) {
                return value;
            }
        }
        return null;
    }
}
