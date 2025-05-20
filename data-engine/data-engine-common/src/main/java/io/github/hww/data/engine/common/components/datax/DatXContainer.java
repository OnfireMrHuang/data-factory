package io.github.hww.data.engine.common.components.datax;

import io.github.hww.data.engine.common.components.datax.utilities.ConfigParser;
import lombok.extern.slf4j.Slf4j;
import com.alibaba.datax.common.util.Configuration;
import com.alibaba.datax.core.Engine;
import org.springframework.stereotype.Component;

import java.util.function.Supplier;

@Slf4j
@Component
public class DatXContainer {

    public void start(Supplier<String> jobContent) {

        Configuration configuration = ConfigParser.parse(jobContent);
        log.info("DataXEngin启动job参数:\n{}", configuration);

        Engine engine = new Engine();
        engine.start(configuration);
    }
}
