package individual.data_foundation.common.spark;

import org.apache.spark.SparkConf;
import org.apache.spark.sql.SparkSession;


public class SparkClientFactoryService {

    public SparkClient newSparkClient(
            String master,
            String appName,
            SparkConf conf) {
        SparkSession spark = SparkSession
                .builder()
                .enableHiveSupport()
                .config(conf)
                .appName(appName)
                .master(master)
                .getOrCreate();
        return new SparkClient(spark, conf);
    }
}
