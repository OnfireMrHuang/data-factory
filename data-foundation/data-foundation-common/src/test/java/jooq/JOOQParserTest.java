package jooq;


import org.jooq.Configuration;
import static org.jooq.impl.DSL.*;
import java.util.List;

import org.jooq.*;
import org.jooq.conf.*;
import org.jooq.impl.*;
import org.junit.Test;

import java.util.ArrayList;

public class JOOQParserTest {
    @Test
    public void testMysqlParser() {
        List<String> parts = new ArrayList<>();
        Configuration configuration = new DefaultConfiguration();
        Query query = configuration
                .derive(VisitListener.onVisitStart(ctx -> {
                    String part = ctx.queryPart().toString();
                    parts.add(part.toString());
                }))
                .dsl()
                .parser()
                .parseQuery("select * from table1 where age > 18");
        // 抽取元信息
        System.out.println(parts);
        // 翻译成具体方言
        System.out.println(DSL.using(SQLDialect.MYSQL).render(query));
    }

    @Test
    public void testPostgresParser() {
        List<String> parts = new ArrayList<>();
        Configuration configuration = new DefaultConfiguration();
        Query query = configuration
                .derive(VisitListener.onVisitStart(ctx -> {
                    String part = ctx.queryPart().toString();
                    parts.add(part.toString());
                }))
                .dsl()
                .parser()
                .parseQuery("select * from table1 where age > 18");
        // 抽取元信息
        System.out.println(parts);
        // 翻译成具体方言
        System.out.println(DSL.using(SQLDialect.POSTGRES).render(query));
    }
}
