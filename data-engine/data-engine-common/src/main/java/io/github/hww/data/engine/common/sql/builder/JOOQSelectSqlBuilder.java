package io.github.hww.data.engine.common.sql.builder;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;
import org.jooq.Record;
import org.jooq.*;
import org.jooq.impl.DSL;

import java.util.List;

@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
public class JOOQSelectSqlBuilder {

    private List<SelectFieldOrAsterisk> fields;

    private List<Table<?>> froms;

    private List<TableJoinRelation> joins;

    private List<Condition> wheres;

    private List<GroupField> groups;

    public String build(SQLDialect sqlDialect) {
        SelectFromStep<Record> select = DSL.using(sqlDialect).select(fields);
        if (froms == null || froms.isEmpty()) {
            return select.getSQL();
        }
        SelectJoinStep<Record> selectFrom = select.from(froms);
        if (joins != null && !joins.isEmpty()) {
            for (TableJoinRelation joinRelation : joins) {
                selectFrom = selectFrom.join(joinRelation.getTable(), joinRelation.getJoinType())
                        .on(joinRelation.getCondition());
            }
        }
        SelectConnectByStep<Record> selectWhere = selectFrom;
        if (wheres != null && !wheres.isEmpty()) {
            selectWhere = selectFrom.where(wheres);
        }
        SelectGroupByStep<Record> selectGroup = selectWhere;
        if(groups == null || groups.isEmpty()) {
            return selectGroup.getSQL();
        }
        return selectGroup.groupBy(groups).getSQL();
    }
}
