package io.github.hww.data.engine.common.sql.parser.model.exp;

import java.util.Map;

public class Case implements Expression {

    private final Map<Expression, Expression> whenThen;
    private final Expression elseExpr;

    public Case(Map<Expression, Expression> whenThen, Expression elseExpr) {
        this.whenThen = whenThen;
        this.elseExpr = elseExpr;
    }

    @Override
    public ExpressionKind kind() {
        return ExpressionKind.Case;
    }

    @Override
    public String getString() {
        return "CASE WHEN " + whenThen.entrySet().stream()
                .map(entry -> entry.getKey().getString() + " THEN " + entry.getValue().getString())
                .reduce((s1, s2) -> s1 + " " + s2)
                .orElse("") + " ELSE " + elseExpr.getString() + " END";
    }
}
