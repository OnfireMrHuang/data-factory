package io.github.hww.data.engine.common.sql.parser.visitor;

import com.alibaba.druid.sql.ast.statement.SQLExprTableSource;
import com.alibaba.druid.sql.dialect.mysql.visitor.MySqlASTVisitorAdapter;
import io.github.hww.data.engine.common.sql.parser.model.SelectQuery;
import lombok.Getter;

@Getter
public class MysqlSelectDruidVisitor extends MySqlASTVisitorAdapter {
    private SelectQuery selectQuery;

    public boolean visit(SQLExprTableSource x) {
        return true;
    }
}
