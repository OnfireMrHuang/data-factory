package io.github.hww.data.engine.common.sql.parser;

import io.github.hww.data.engine.common.antlr4.mysql.MySqlParser;
import io.github.hww.data.engine.common.antlr4.mysql.MySqlLexer;
import io.github.hww.data.engine.common.sql.parser.model.SelectQuery;
import io.github.hww.data.engine.common.sql.parser.visitor.MysqlSelectAntlr4Visitor;
import org.antlr.v4.runtime.CharStreams;
import org.antlr.v4.runtime.CommonTokenStream;
import org.antlr.v4.runtime.ParserRuleContext;

import java.util.function.BiConsumer;
import java.util.function.Function;

public final class Antlr4SqlParseUtils {

    private static final BiConsumer<MySqlLexer, MySqlParser> initializer = new RefreshableParserInitializer<MySqlLexer, MySqlParser>(){};

    public static SelectQuery parseMysqlSelect(String sql) {
        final Function<MySqlParser, ParserRuleContext> parseFunction = MySqlParser::selectStatement;

        MySqlLexer mysqlLexer = null;
        MySqlParser mysqlParser = null;
        CommonTokenStream commonTokenStream = null;
        try {
            mysqlLexer = new MySqlLexer(new CaseInsensitiveStream(CharStreams.fromString(sql)));
            commonTokenStream = new CommonTokenStream(mysqlLexer);
            mysqlParser = new MySqlParser(commonTokenStream);

            cacheLexerAndParser(mysqlLexer, mysqlParser);

            ParserRuleContext tree = parseFunction.apply(mysqlParser);

            return new MysqlSelectAntlr4Visitor().visit(tree);
        } finally {
            if(mysqlParser != null) {
                mysqlParser.reset();
                mysqlParser = null;
            }
            if(mysqlLexer != null) {
                mysqlLexer.reset();
                mysqlLexer = null;
            }
            if(commonTokenStream != null) {
                commonTokenStream.seek(0);
                commonTokenStream = null;
            }
        }
    }

    private static void cacheLexerAndParser(MySqlLexer mySqlLexer, MySqlParser mySqlParser) {
        initializer.accept(mySqlLexer, mySqlParser);
    }
}
