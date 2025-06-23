package io.github.hww.data.engine.common.sql.parser.model.exp;

public class Bool implements Expression {
    private final Expression left;
    private final BooleanOperator booleanOperator;
    private final Expression right;

    public Bool(Expression left, BooleanOperator booleanOperator, Expression right) {
        this.left = left;
        this.booleanOperator = booleanOperator;
        this.right = right;
    }

    public Bool(Expression left, BooleanOperator booleanOperator) {
        this(left, booleanOperator, null);
    }

    @Override
    public ExpressionKind kind() {
        return ExpressionKind.Boolean;
    }

    @Override
    public String getString() {
        return switch (booleanOperator) {
            case IsNull, IsNotNull -> left.getString() + " " + booleanOperator.name();
            case Between-> String.format("BETWEEN %s AND %s", left.getString(), right.getString());
            case NotBetween -> String.format("NOT BETWEEN %s AND %s", left.getString(), right.getString());
            default -> left.getString() + " " + booleanOperator.name() + " " + right.getString();
        };
    }
}
