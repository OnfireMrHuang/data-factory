package io.github.hww.data.engine.common.sql.parser.model;

import lombok.Data;

import java.util.List;

@Data
public class SelectQuery {

    /**
     * 联合查询操作符: union, union all, except, except all, intersect, intersect all
     */
    private String combineOperator;

    /**
     * 单个查询
     */
    private List<SingleSelect> singleSelects;
}
