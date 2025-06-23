package io.github.hww.data.engine.common.sql.parser.model.exp;

public enum OrderType {

    ASC("asc"),
    DESC("desc");

    private final String orderType;

    OrderType(String orderType) {
        this.orderType = orderType;
    }

    public String getOrderType() {
        return orderType;
    }

    public static OrderType fromValue(String orderType) {
        for (OrderType value : values()) {
            if (value.orderType.equals(orderType)) {
                return value;
            }
        }
        return null;
    }
}
