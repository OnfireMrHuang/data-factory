package io.github.hww.data.engine.common.ingestion;

import com.alibaba.datax.common.util.Configuration;
import com.alibaba.datax.core.Engine;

public final class DataXFullLoad {

    // 根据datax job content启动DataX
    public void start(String jobContent) {
        Configuration configuration = Configuration.from(jobContent);

        Engine engine = new Engine();
        engine.start(configuration);
    }
}
