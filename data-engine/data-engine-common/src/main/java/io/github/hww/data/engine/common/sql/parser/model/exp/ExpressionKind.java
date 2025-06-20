package io.github.hww.data.engine.common.sql.parser.model.exp;

public enum ExpressionKind {
    Column("column"),
    Boolean("boolean"),
    Literal("literal");

    private final String expressionKind;

    ExpressionKind(String kind) {
        this.expressionKind = kind;
    }

    public static ExpressionKind fromValue(String expressionKind) {
        for (ExpressionKind kind : ExpressionKind.values()) {
            if (kind.expressionKind.equals(expressionKind)) {
                return kind;
            }
        }
        return Literal;
    }
}
