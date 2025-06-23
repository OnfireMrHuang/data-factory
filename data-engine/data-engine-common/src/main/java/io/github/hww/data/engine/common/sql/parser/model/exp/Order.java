package io.github.hww.data.engine.common.sql.parser.model.exp;

public class Order implements Expression {

    private final Expression expression;
    private final OrderType orderType;

    public Order(Expression expression, OrderType orderType) {
        this.expression = expression;
        this.orderType = orderType;
    }


    @Override
    public ExpressionKind kind() {
        return ExpressionKind.Order;
    }

    @Override
    public String getString() {
        return expression.getString() + " " + orderType.getOrderType();
    }
}
