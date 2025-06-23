package io.github.hww.data.engine.common.sql.parser.model.exp;

public enum ExpressionKind {
    Alias("alias"),
    Column("column"),
    Boolean("boolean"),
    Condition("condition"),
    Case("case"),
    Func("func"),
    Order("order"),
    Window("window"),
    Star("star"),
    Value("value"),
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
