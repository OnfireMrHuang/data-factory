
import org.junit.Test;

import java.sql.*;
import java.util.Properties;

import static org.junit.Assert.*;

public class MySQLConnectTest {
    @Test
    public void testConnection() {
        try {

            // Connection conn = DriverManager.getConnection("jdbc:mysql://audit.mingyuanyun.com:30268/bigdata_config",
            //         "dffc5533-a60d-4d33-a847-5a368db7bab1", "MLSOoqE1gStF9a1i");

            // 建立数据库连接
            Properties prop = new Properties();
            prop.put("user", "dffc5533-a60d-4d33-a847-5a368db7bab1");
            prop.put("password", "MLSOoqE1gStF9a1i");
            Class.forName("com.mysql.cj.jdbc.Driver");
            Connection conn = DriverManager.getConnection(url, prop);
            assertNotNull(conn);
            Statement statement = conn.createStatement();

            // 执行select SQL语句并打印结果
            ResultSet resultSet = statement.executeQuery("SELECT JSON_EXTRACT('{\"val\":\"测试xxxx\"}','$.val') as test FROM project_main");

            // 打印查询结果
            while (resultSet.next()) {
                String testStr = resultSet.getString("test");
                System.out.println("Values: " + testStr);
            }

            conn.close();
        } catch ( SQLException e) {
            fail("数据库连接失败: " + e.getMessage());
        } catch (ClassNotFoundException e) {
            throw new RuntimeException(e);
        }
    }
}