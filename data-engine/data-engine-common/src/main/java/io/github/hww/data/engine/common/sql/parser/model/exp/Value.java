package io.github.hww.data.engine.common.sql.parser.model.exp;

public class Value implements Expression{

    private final String value;

    public Value(String value) {
        this.value = value;
    }

    @Override
    public ExpressionKind kind() {
        return ExpressionKind.Value;
    }

    @Override
    public String getString() {
        return String.format("'%s'", value);
    }
}
