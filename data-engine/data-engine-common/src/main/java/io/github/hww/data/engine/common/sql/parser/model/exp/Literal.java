package io.github.hww.data.engine.common.sql.parser.model.exp;

import java.util.List;

public class Literal implements Expression {

    private String literal;

    public ExpressionKind kind() {
        return ExpressionKind.Literal;
    }

    public String getString() {
        return literal;
    }
}
