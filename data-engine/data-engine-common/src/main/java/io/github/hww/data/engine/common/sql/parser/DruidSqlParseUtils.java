package io.github.hww.data.engine.common.sql.parser;

import com.alibaba.druid.sql.SQLUtils;
import com.alibaba.druid.sql.ast.SQLStatement;
import com.alibaba.druid.util.JdbcConstants;
import io.github.hww.data.engine.common.sql.parser.model.SelectQuery;
import io.github.hww.data.engine.common.sql.parser.visitor.MysqlSelectDruidVisitor;

import java.util.List;

public final class DruidSqlParseUtils {
    public SelectQuery parseMysqlSelect(String sql) {
        String dbType = String.valueOf(JdbcConstants.MYSQL);
        List<SQLStatement> statementList = SQLUtils.parseStatements(sql, dbType);

        MysqlSelectDruidVisitor visitor = new MysqlSelectDruidVisitor();
        for (SQLStatement stmt : statementList) {
            stmt.accept(visitor);
        }
        return visitor.getSelectQuery();
    }
}
