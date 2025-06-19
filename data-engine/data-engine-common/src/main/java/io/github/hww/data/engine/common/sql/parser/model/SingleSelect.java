package io.github.hww.data.engine.common.sql.parser.model;

import lombok.Data;

import java.util.List;


@Data
public class SingleSelect {

    private boolean setDistinct;

    private List<Expression> selectExpressions;

    private TableSource fromTableSource;

    private List<JoinTableSource> joinTables;

    private Condition where;

    private List<Expression> groupByExpressions;

    private Condition having;

    private List<Expression> orderByExpressions;

    private Integer offset;

    private Integer limit;
}
