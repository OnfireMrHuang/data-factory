package io.github.hww.data.engine.common.sql.parser.model.exp;

public interface Expression {
    ExpressionKind kind();
    String getString();
}
