package io.github.hww.data.foundation.common;


import io.github.hww.data.foundation.common.components.storages.minio.MinioStorage;
import io.github.hww.data.foundation.common.config.properties.MinioProperties;
import io.minio.messages.Item;
import lombok.extern.slf4j.Slf4j;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.springframework.boot.context.properties.EnableConfigurationProperties;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.test.context.TestPropertySource;
import org.springframework.test.context.junit4.SpringRunner;

import javax.annotation.Resource;
import java.util.List;

@RunWith(SpringRunner.class)
@SpringBootTest(classes={MinioProperties.class,MinioStorage.class})
@EnableConfigurationProperties
@TestPropertySource(properties = {
        "minio.endpoint=xxx",
        "minio.access_key=xxx",
        "minio.access_secret=xxx",
        "minio.bucket=xxx",
        "minio.region=cn-south-1"
})
@Slf4j
public class MinioTest {

    @Resource
    private MinioStorage minioStorage;

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
