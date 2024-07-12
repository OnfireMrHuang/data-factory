package individual.data_foundation.common.spark;

import org.apache.spark.SparkConf;
import org.apache.spark.api.java.JavaSparkContext;
import org.apache.spark.scheduler.SparkListener;
import org.apache.spark.sql.Dataset;
import org.apache.spark.sql.Row;
import org.apache.spark.sql.SQLContext;
import org.apache.spark.sql.SparkSession;
import org.apache.spark.sql.streaming.StreamingQueryListener;
import org.apache.spark.sql.util.QueryExecutionListener;


public class SparkClient {

    private SparkSession spark;
    private SparkConf conf;

    public SparkClient(SparkSession spark, SparkConf conf) {
        this.spark = spark;
        this.conf = conf;
    }

    public void registerQueryExecutionListener(
            QueryExecutionListener listener) {
        spark.listenerManager().register(listener);
    }

    public void registerSparkListener(
            SparkListener listener) {
        spark.sparkContext().addSparkListener(listener);
    }

    public void registerStreamingQueryListener(
            StreamingQueryListener listener) {
        spark.streams().addListener(listener);
    }

    public SparkConf getConf() {
        return conf;
    }


    public SparkSession getSpark() {
        return spark;
    }

    public JavaSparkContext toJavaSparkContext() {
        return new JavaSparkContext(conf);
    }

    public SQLContext toSQLContext() {
        return new SQLContext(toJavaSparkContext());
    }

    public void close() {
        spark.close();
    }

    public Dataset<Row> sql(String sql) {
        Dataset<Row> dataset = this.spark.sql(sql);
        return dataset;
    }
}