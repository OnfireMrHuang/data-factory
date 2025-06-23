package io.github.hww.data.engine.common.sql.parser.model.exp;

public class Alias implements Expression {
    private final Expression aliased;
    private final String alias;

    public Alias(Expression aliased, String alias) {
        this.aliased = aliased;
        this.alias = alias;
    }

    @Override
    public ExpressionKind kind() {
        return ExpressionKind.Alias;
    }

    @Override
    public String getString() {
        return String.format("%s as %s", aliased.getString(), alias);
    }
}
