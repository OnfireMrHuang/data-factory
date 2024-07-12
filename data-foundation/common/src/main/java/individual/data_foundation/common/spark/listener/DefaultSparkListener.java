package individual.data_foundation.common.spark.listener;

import org.apache.spark.scheduler.SparkListener;
import org.apache.spark.scheduler.SparkListenerTaskEnd;
import org.apache.spark.scheduler.SparkListenerTaskStart;

public class DefaultSparkListener extends SparkListener {

    @Override
    public void onTaskStart(SparkListenerTaskStart taskStart) {
        super.onTaskStart(taskStart);
    }

    @Override
    public void onTaskEnd(SparkListenerTaskEnd taskEnd) {
        super.onTaskEnd(taskEnd);
    }
}
