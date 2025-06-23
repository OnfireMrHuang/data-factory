package io.github.hww.data.engine.common.sql.parser.model.exp;

public class Col implements Expression {
    private String schema;
    private String table;
    private final String column;

    public Col(String schema, String table, String column) {
        this.schema = schema;
        this.table = table;
        this.column = column;
    }

    public Col(String table, String column) {
        this.table = table;
        this.column = column;
    }

    public Col(String column) {
        this.column = column;
    }

    @Override
    public ExpressionKind kind() {
        return ExpressionKind.Column;
    }

    @Override
    public String getString() {
        if (schema != null) {
            return schema + "." + table + "." + column;
        } else if (table != null) {
            return table + "." + column;
        } else {
            return column;
        }
    }
}
