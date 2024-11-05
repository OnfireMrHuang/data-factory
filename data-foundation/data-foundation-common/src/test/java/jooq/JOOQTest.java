package jooq;

import org.jooq.DSLContext;
import org.jooq.Query;
import org.jooq.SQLDialect;
import org.jooq.conf.ParamType;
import org.jooq.impl.DSL;
import org.junit.Test;

import static org.jooq.DatePart.DAY;
import static org.jooq.DatePart.MONTH;
import static org.jooq.impl.DSL.*;
import static org.jooq.impl.SQLDataType.DATE;
import static org.jooq.impl.SQLDataType.INTEGER;

public class JOOQTest {
    @Test
    public void testJOOQMysql() {
        // 创建 DSLContext
        DSLContext create = DSL.using(SQLDialect.MYSQL);

        // 子查询: union,where, 日期函数、转换函数
        // 主体sql： join、order by、limit、offset、subQuery and so on
        Query query = create.select(field("users.name"),
                        sum(field("orders.amount").cast(INTEGER)).as("total_amount"),
                        count(field("orders.id")).as("total_order_count"))
                .from(table("users"))
                .leftJoin(table(select(field("id"),field("user_id"), field("brand"), field("model_no"), field("amount").cast(INTEGER))
                        .from(table("phone_sale_orders"))
                        .where(dateDiff(DAY, currentDate(), field("sell_time",DATE)).gt(30))
                        .union(
                                select(field("id"), field("user_id"),field("brand"), field("model_no"), field("amount").cast(INTEGER))
                                        .from(table("computer_sale_orders"))
                                        .where(dateDiff(MONTH, currentDate(), field("sell_time",DATE)).lt(30))
                        )).as("orders"))
                .on(field("users.id").eq(field("orders.user_id")))
                .where(field("users.age").gt(18)).and(field("users.gender").eq("M"))
                .orderBy(field("orders.amount").desc()).
                offset(1).limit(100);

        String sql = query.getSQL(ParamType.INLINED);
        System.out.println(sql);
    }

    @Test
    public void testJOOPPostgres() {
        // 创建 DSLContext
        DSLContext create = DSL.using(SQLDialect.POSTGRES);

        // 子查询: union,where, 日期函数、转换函数
        // 主体sql： join、order by、limit、offset、subQuery and so on
        Query query = create.select(field("users.name"),
                        sum(field("orders.amount").cast(INTEGER)).as("total_amount"),
                        count(field("orders.id")).as("total_order_count"))
                .from(table("users"))
                .leftJoin(table(select(field("id"),field("user_id"), field("brand"), field("model_no"), field("amount").cast(INTEGER))
                        .from(table("phone_sale_orders"))
                        .where(dateDiff(DAY, currentDate(), field("sell_time",DATE)).gt(30))
                        .union(
                                select(field("id"), field("user_id"),field("brand"), field("model_no"), field("amount").cast(INTEGER))
                                        .from(table("computer_sale_orders"))
                                        .where(dateDiff(MONTH, currentDate(), field("sell_time",DATE)).lt(30))
                        )).as("orders"))
                .on(field("users.id").eq(field("orders.user_id")))
                .where(field("users.age").gt(18)).and(field("users.gender").eq("M"))
                .orderBy(field("orders.amount").desc()).
                offset(1).limit(100);

        String sql = query.getSQL(ParamType.INLINED);
        System.out.println(sql);
    }
}
