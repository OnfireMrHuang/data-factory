package io.github.hww.data.engine.common;

import io.github.hww.data.engine.common.sql.builder.JOOQSelectSqlBuilder;
import io.github.hww.data.engine.common.sql.builder.TableJoinRelation;
import org.jooq.SQLDialect;
import org.jooq.*;
import org.jooq.impl.DSL;
import org.junit.Test;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class SqlBuilderTest {

    @Test
    public void testSqlBuilder() {
        List<SelectFieldOrAsterisk> selectField = new ArrayList<>(); // select
        Table<?> tableSource = null; // from
        List<TableJoinRelation> joins = new ArrayList<>(); // join
        List<Condition> conditions = new ArrayList<>(); // where

        selectField.add(DSL.field("a.id"));
        selectField.add(DSL.field("b.name"));
        tableSource = DSL.table("a");
        joins.add(TableJoinRelation.builder()
                .table(DSL.table("b"))
                .joinType(JoinType.JOIN)
                .condition(DSL.field("a.id").eq(DSL.field("b.id")))
                .build());
        conditions.add(DSL.condition("a.id is not null"));

        JOOQSelectSqlBuilder builder = JOOQSelectSqlBuilder.builder()
                .fields(selectField)
                .froms(Collections.singletonList(tableSource))
                .joins(joins)
                .wheres(conditions).build();
        String sql = builder.build(SQLDialect.MYSQL);
        System.out.println(sql);
    }
}
