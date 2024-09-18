package io.github.hww.data.foundation;


import lombok.extern.slf4j.Slf4j;

import org.mybatis.spring.annotation.MapperScan;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@Slf4j
@SpringBootApplication
@MapperScan("io.github.hww.data.foundation.dao.mapper") // 指定mapper所在的包
public class Main {

    public static void main(String[] args) {
        // 正式启动应用
        SpringApplication.run(Main.class, args);
    }
}