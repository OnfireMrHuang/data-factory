package io.github.hww.data.engine.common.ingestion;

import org.junit.Test;

public class dataXTest {
    @Test
    public void test_datax() {
        String path = "/stream2stream.json";
        DataXFullLoad dataXFullLoad = new DataXFullLoad();
        dataXFullLoad.start(this.getClass().getResource(path).getPath());
    }
}
