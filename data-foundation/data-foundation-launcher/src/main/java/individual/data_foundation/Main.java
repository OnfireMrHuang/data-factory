package individual.data_foundation;

import lombok.extern.slf4j.Slf4j;

import org.mybatis.spring.annotation.MapperScan;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@Slf4j
@SpringBootApplication
@MapperScan("individual.data_foundation.dao.privilege.database.mapper") // 指定mapper所在的包
public class Main {

    public static void main(String[] args) {
        log.info("启动数据基座项目!");
        SpringApplication.run(Main.class, args);
    }
}