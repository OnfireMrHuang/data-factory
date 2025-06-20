package io.github.hww.data.engine.common.sql.parser.model;

import io.github.hww.data.engine.common.sql.parser.model.exp.Expression;
import lombok.Data;

import java.util.List;


@Data
public class SingleSelect {

    private Boolean distinct;

    private List<Expression> selectColumns;

    private TableSource from;

    private List<JoinTableSource> joins;

    private Expression where;

    private List<Expression> groupBy;

    private Expression having;

    private List<Expression> order;

    private Integer offset;

    private Integer limit;

    private List<Expression> windows;
}
