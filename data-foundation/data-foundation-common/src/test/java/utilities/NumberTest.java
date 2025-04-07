package utilities;

import org.junit.Test;

public class NumberTest {

    @Test
    public void TestStringHash() {
        String tableName = "dwd_gc_zlgl_special_inspect_sfsfsfsfcheck_quality_batch_concern";
        int hash = tableName.hashCode();
        String newTableName = tableName + "_" + Math.abs(hash % 1000); // 取哈希值的绝对值并限制长度
        if(newTableName.length() > 64) {
            System.out.println("newTableName is too long");
        }
        System.out.println(newTableName);
    }
}
