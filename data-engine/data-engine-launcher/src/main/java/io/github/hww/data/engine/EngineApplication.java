package io.github.hww.data.engine;



import org.mybatis.spring.annotation.MapperScan;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
@MapperScan("io.github.hww.data.engine.dao.mapper") // 指定mapper所在的包
public class EngineApplication {

    public static void main(String[] args) {
        // 正式启动应用
        SpringApplication.run(EngineApplication.class, args);
    }
}