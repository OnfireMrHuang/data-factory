package io.github.hww.data.engine.common.sql.parser.model.exp;

public class Literal implements Expression {

    private final String literal;

    public Literal(String literal) {
        this.literal = literal;
    }

    public ExpressionKind kind() {
        return ExpressionKind.Literal;
    }

    public String getString() {
        return literal;
    }
}
