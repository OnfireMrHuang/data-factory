package io.github.hww.data.foundation.common;


import io.github.hww.data.foundation.common.components.storages.minio.MinioStorage;
import io.minio.MinioProperties;
import io.minio.messages.Item;
import lombok.extern.slf4j.Slf4j;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.test.context.junit4.SpringRunner;

import javax.annotation.Resource;
import java.util.List;

@RunWith(SpringRunner.class)
@SpringBootTest(classes={MinioStorage.class, MinioProperties.class})
@Slf4j
public class MinioTest {

    @Resource
    private MinioStorage minioStorage;

    @Before
    public void init() {
        // 设置MinoProperties的环境变量
        System.setProperty("mino.endpoint", "https://obs.cn-south-1.myhuaweicloud.com");
        System.setProperty("mino.access_key", "DYOKHWKGBCQDHV3O5UPM");
        System.setProperty("mino.access_secret", "6ymRQaDeor12zYR6M8Nl2UDspP7kHcX5RIXAOhKs");
        System.setProperty("mino.bucket", "dap-test-new");
        System.setProperty("mino.region", "cn-south-1");
    }

    @Test
    public void testMinioListObjects() {
        try {
            List<Item> items = minioStorage.listObjects(null, true, 10);
            for (Item item : items) {
                System.out.println(item.objectName());
            }
        } catch (Exception e) {
            log.error(e.getMessage(), e);
        }
    }
}
