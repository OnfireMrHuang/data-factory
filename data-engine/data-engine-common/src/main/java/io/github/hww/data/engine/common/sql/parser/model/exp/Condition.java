package io.github.hww.data.engine.common.sql.parser.model.exp;

import java.util.List;

public class Condition implements Expression {
    private final ConditionOperator operator;
    private final List<Expression> groupItems;

    public Condition(ConditionOperator operator, List<Expression> groupItems) {
        this.operator = operator;
        this.groupItems = groupItems;
    }

    @Override
    public ExpressionKind kind() {
        return ExpressionKind.Condition;
    }

    @Override
    public String getString() {
        return switch (operator) {
            case And -> groupItems.stream().map(Expression::getString).reduce((a, b) -> a + " AND " + b).get();
            case Or -> groupItems.stream().map(Expression::getString).reduce((a, b) -> a + " OR " + b).get();
            default -> null;
        };
    }
}
