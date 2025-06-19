package io.github.hww.data.engine.common.sql.parser;

import com.alibaba.druid.sql.SQLUtils;
import com.alibaba.druid.sql.ast.SQLStatement;
import com.alibaba.druid.util.JdbcConstants;

import java.util.List;

public final class DruidSqlParseUtils {
    public void parseMysql(String sql) {

    }
    public List<String> parseMysqlTables(String sql) {
        String dbType = String.valueOf(JdbcConstants.MYSQL);
        List<SQLStatement> statementList = SQLUtils.parseStatements(sql, dbType);

    }
}
