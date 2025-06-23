package io.github.hww.data.engine.common.sql.parser.model.exp;

import java.util.List;

public class Func implements Expression {

    private final String funcName;
    private final List<Expression> args;

    public Func(String funcName, List<Expression> args) {
        this.funcName = funcName;
        this.args = args;
    }


    @Override
    public ExpressionKind kind() {
        return ExpressionKind.Func;
    }

    @Override
    public String getString() {
        return funcName + "(" + String.join(", ", args.stream().map(Expression::getString).toList()) + ")";
    }
}
