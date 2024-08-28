import cn.hutool.core.io.FileUtil;
import org.junit.Test;

import java.io.*;
import java.nio.file.*;
import cn.hutool.core.util.ZipUtil;

public class ZipTest {
    @Test
    public void testZip() {
        String rootPath = "/Users/huangwenwu/mingyuanyun/erp";
        String zipFilePath = rootPath + "/5.0.9.2.zip";
        String subDirName = "data/bigdata/product_public_space/main"; // 要抽取的子目录名
        String newFilePath = rootPath + "/5.0.9.2.normal"; // 新 ZIP 文件路径

        try {
            File unzipFile = ZipUtil.unzip(zipFilePath);

            // 将subDir移动到rootPath
            File subDir = new File(unzipFile, subDirName);
            Path subDirPath = subDir.toPath();
            // 将subDirPath移动到newFilePath
            Files.move(subDirPath, Paths.get(newFilePath), StandardCopyOption.REPLACE_EXISTING);

            // 将移动之后的文件进行压缩
            ZipUtil.zip(newFilePath);
            FileUtil.del(unzipFile);
        } catch (Exception e) {
            e.printStackTrace();
        } finally {
            FileUtil.del(newFilePath);
        }
    }
}
