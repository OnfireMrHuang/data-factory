package io.github.hww.data.engine.common;

import io.github.hww.data.engine.common.datax.DataXEngine;
import org.apache.commons.lang3.StringUtils;
import org.junit.Test;

import java.io.File;


public class DataXEngineTest {

    @Test
    public void testMysqlToDoris() {
        // 在你的IDE上设置实际的datax.home和datax.testJobPath环境变量上去然后运行
        String dataxHomePath = System.getenv("datax.home");
        if (StringUtils.isEmpty(dataxHomePath) || !new File(dataxHomePath).exists()) {
            throw new RuntimeException("datax.home is not exists");
        }
        System.out.println("datax.home:" + dataxHomePath);

        String testJobPath = System.getenv("datax.testJobPath");
        if  (StringUtils.isEmpty(testJobPath) || !new File(testJobPath).exists()) {
            throw new RuntimeException("datax.testJobPath is not exists");
        }
        System.out.println("datax.testJobPath:" + testJobPath);

        DataXEngine engine = new DataXEngine();
        engine.start(testJobPath);
    }
}


