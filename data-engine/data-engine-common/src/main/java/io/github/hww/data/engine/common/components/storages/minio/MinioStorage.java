package io.github.hww.data.engine.common.components.storages.minio;

import cn.hutool.core.util.URLUtil;
import com.google.common.collect.Lists;
import io.github.hww.data.engine.common.config.properties.MinioProperties;
import io.github.hww.data.engine.common.models.components.StorageException;
import io.minio.*;
import io.minio.credentials.StaticProvider;
import io.minio.messages.Item;
import io.minio.messages.LifecycleConfiguration;
import lombok.extern.slf4j.Slf4j;
import org.apache.commons.lang3.StringUtils;
import org.apache.commons.lang3.exception.ExceptionUtils;
import org.springframework.stereotype.Component;

import java.io.File;
import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.InputStream;
import java.net.URL;
import java.time.ZonedDateTime;
import java.util.List;
import java.util.Map;

@Slf4j
@Component
public class MinioStorage {

    private final MinioProperties minioConfig;
    private final StaticProvider sp;

    private MinioClient minioClient;

    public MinioStorage(MinioProperties minioConfig) {
        this.minioConfig = minioConfig;
        sp = new StaticProvider(minioConfig.getAccessKey(), minioConfig.getAccessSecret(), null);
        try {
            minioClient = MinioClient.builder()
                    .endpoint(minioConfig.getEndpoint())
                    .credentials(minioConfig.getAccessKey(), minioConfig.getAccessSecret())
                    .build();// new MinioClient(endpoint, accessKey, accessSecret);
        } catch (Exception e) {
            log.error("无效的endpoint {}", minioConfig.getEndpoint(), e);
        }
    }

    /**
     * 判断bucket是否存在
     *
     * @param bucket 桶名称
     * @return true 存在 false 不存在
     */
    public boolean bucketExists(String bucket) {
        BucketExistsArgs existsArgs = BucketExistsArgs.builder()
                .region(minioConfig.getRegion())
                .bucket(bucket).build();
        try {
            return minioClient.bucketExists(existsArgs);
        } catch (Exception e) {
            log.error("判断bucket是否存在异常 {}", e.getMessage(), e);
        }
        return false;
    }

    /**
     * 获取bucket生命周期配置
     *
     * @return LifecycleConfiguration
     * @throws StorageException 存储异常
     */
    public LifecycleConfiguration getBucketLifecycle() throws StorageException {
        GetBucketLifecycleArgs getBucketLifecycleArgs = GetBucketLifecycleArgs.builder().
                bucket(minioConfig.getBucket()).region(minioConfig.getRegion()).build();
        try {
            return minioClient.getBucketLifecycle(getBucketLifecycleArgs);
        } catch (Exception e) {
            log.error("获取bucket生命周期配置异常{}", e.getMessage(), e);
            throw new StorageException(e.getMessage(), ExceptionUtils.getStackTrace(e));
        }
    }

    /**
     * 设置bucket生命周期配置
     *
     * @param config LifecycleConfiguration
     * @throws StorageException 存储异常
     */
    private void setBucketLifecycle(LifecycleConfiguration config) throws StorageException {
        SetBucketLifecycleArgs setBucketLifecycleArgs = SetBucketLifecycleArgs.builder()
                .bucket(minioConfig.getBucket()).region(minioConfig.getRegion()).config(config).build();
        try {
            log.info("设置bucket生命周期配置: {}", config);
            minioClient.setBucketLifecycle(setBucketLifecycleArgs);
        }  catch (Exception e) {
            log.error("设置bucket生命周期配置异常{}", e.getMessage(), e);
            throw new StorageException(e.getMessage(), ExceptionUtils.getStackTrace(e));
        }
    }


    /**
     * 列出指定bucket下的所有文件
     *
     * @param prefix    前缀
     * @param recursive 是否递归查找，false则只查找文件夹下的第一层文件
     * @param n         1~1000
     * @return List<Item> 对象列表
     * @throws StorageException 存储异常
     */
    public List<Item> listObjects(String prefix, Boolean recursive, Integer n) throws StorageException {
        List<Item> response = Lists.newArrayList();
        ListObjectsArgs args = ListObjectsArgs.builder()
                .bucket(minioConfig.getBucket())
                .region(minioConfig.getRegion())
                .recursive(recursive)
                .prefix(prefix)
                .maxKeys(n)
                .build();
        Iterable<Result<Item>> objects = minioClient.listObjects(args);
        try {
            for (Result<Item> result : objects) {
                Item item = result.get();
                response.add(item);
            }
        } catch (Exception e) {
            log.error(e.getMessage(), e);
            throw new StorageException(e.getMessage(), ExceptionUtils.getStackTrace(e));
        }
        return response;
    }

    /**
     * 获取文件信息
     *
     * @param objectKey 文件名称
     * @return StatObjectResponse
     * @throws StorageException 存储异常
     */
    public StatObjectResponse statObject(String objectKey) throws StorageException {
        StatObjectArgs objectArgs = StatObjectArgs.builder().bucket(minioConfig.getBucket())
                .region(minioConfig.getRegion())
                .object(objectKey)
                .build();
        StatObjectResponse response = null;
        try {
            response = minioClient.statObject(objectArgs);
        } catch (Exception e) {
            if (!e.toString().contains("NoSuchKey")) {
                throw new StorageException(e.getMessage(), ExceptionUtils.getStackTrace(e));
            }
        }
        return response;
    }

    /**
     * 下载文件
     *
     * @param filePath  文件路径
     * @param objectKey 文件名称
     * @throws StorageException 存储异常
     */
    public void fGetObject(String filePath, String objectKey) throws StorageException {
        DownloadObjectArgs args = DownloadObjectArgs.builder()
                .bucket(minioConfig.getBucket())
                .object(objectKey)
                .region(minioConfig.getRegion())
                .filename(filePath)
                .build();
        try {
            minioClient.downloadObject(args);
        } catch (Exception e) {
            log.error(e.getMessage(), e);
            throw new StorageException(e.getMessage(), ExceptionUtils.getStackTrace(e));
        }
    }

    /**
     * 上传文件
     *
     * @param filePath  文件路径
     * @param objectName 文件名称
     * @throws StorageException 存储异常
     */
    public void fPutObject(String filePath, String objectName) throws StorageException {
        File f = new File(filePath);
        try (InputStream basi = new FileInputStream(f)) {
            PutObjectArgs objectArgs = PutObjectArgs.builder()
                    .bucket(minioConfig.getBucket())
                    .object(objectName)
                    .region(minioConfig.getRegion())
                    .stream(basi, basi.available(), -1)
                    .build();
            minioClient.putObject(objectArgs);
        } catch (FileNotFoundException e) {
            log.error(e.getMessage(), e);
            throw new StorageException("文件不存在", ExceptionUtils.getStackTrace(e));
        } catch (Exception e) {
            log.error(e.getMessage(), e);
            throw new StorageException(e.getMessage(), ExceptionUtils.getStackTrace(e));
        }
    }


    /**
     * 上传文件(文件流形式)
     *
     * @param fileInputStream 文件流
     * @param objectName      文件名称
     * @throws StorageException 存储异常
     */
    public void fPutObject(InputStream fileInputStream, String objectName) throws StorageException {
        try {
            PutObjectArgs objectArgs = PutObjectArgs.builder()
                    .bucket(minioConfig.getBucket())
                    .object(objectName)
                    .region(minioConfig.getRegion())
                    .stream(fileInputStream, fileInputStream.available(), -1)
                    .build();
            minioClient.putObject(objectArgs);
        } catch (FileNotFoundException e) {
            log.error(e.getMessage(), e);
            throw new StorageException("文件不存在", ExceptionUtils.getStackTrace(e));
        } catch (Exception e) {
            log.error(e.getMessage(), e);
            throw new StorageException(e.getMessage(), ExceptionUtils.getStackTrace(e));
        }
    }


    /**
     * 获取文件上传链接
     *
     * @param objectName 文件名称
     * @param expires    过期时间
     * @return String
     * @throws StorageException 存储异常
     */
    public String preSignedPutObject(String objectName, Integer expires) throws StorageException {
        boolean isAliOss = minioConfig.getEndpoint().contains("aliyuncs.com");
        PostPolicy policy = new PostPolicy(minioConfig.getBucket(), ZonedDateTime.now().minusSeconds(expires));
        policy.addEqualsCondition("key", objectName);
        Map<String, String> formData;
        try {
            if (isAliOss)
                formData = policy.formData(sp.fetch(), minioConfig.getRegion());
            else
                formData = minioClient.getPresignedPostFormData(policy);// bucket, objectName, expires);
        } catch (Exception e) {
            log.error(e.getMessage(), e);
            throw new StorageException(e.getMessage(), ExceptionUtils.getStackTrace(e));
        }
        StringBuilder builder = new StringBuilder();
        String tmpEndpoint = minioConfig.getEndpoint();
        if (isAliOss) {
            URL url = URLUtil.url(tmpEndpoint);
            String host = url.getHost();
            if (!host.startsWith(minioConfig.getBucket())) host = minioConfig.getBucket() + "." + host;
            tmpEndpoint = url.getProtocol() + "://" + host;
        }
        builder.append(tmpEndpoint);
        if (!StringUtils.endsWithIgnoreCase(tmpEndpoint, "/")) {
            builder.append("/");
        }
        if (!isAliOss) {
            builder.append(minioConfig.getBucket());
            builder.append("/");
        }
        builder.append(objectName);
        builder.append("?");
        formData.forEach((k, v) -> {
            builder.append(String.format("%s=%s", k, v));
            builder.append('&');
        });
        builder.replace(builder.lastIndexOf("&"), builder.lastIndexOf("&") + 1, "");
        return builder.toString();
    }
}
