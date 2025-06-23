package io.github.hww.data.engine.common.sql.parser.model.exp;

public class Star implements Expression {
    @Override
    public ExpressionKind kind() {
        return ExpressionKind.Star;
    }

    @Override
    public String getString() {
        return "*";
    }
}
