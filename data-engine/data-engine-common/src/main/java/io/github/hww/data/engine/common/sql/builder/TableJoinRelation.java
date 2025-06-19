package io.github.hww.data.engine.common.sql.builder;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;
import org.jooq.Condition;
import org.jooq.JoinType;
import org.jooq.TableLike;

@Data
@Builder
@NoArgsConstructor
@AllArgsConstructor
public class TableJoinRelation {

    private TableLike<?> table;

    private JoinType joinType;

    private Condition condition;

}
