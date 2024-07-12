package individual.data_foundation.common.spark.listener;

import org.apache.spark.sql.execution.QueryExecution;
import org.apache.spark.sql.util.QueryExecutionListener;


public class DefaultQueryExecutionListener implements QueryExecutionListener {

    @Override
    public void onSuccess(String funcName, QueryExecution qe, long durationNs) {
        System.out.println("");
    }

    @Override
    public void onFailure(String funcName, QueryExecution qe, Exception exception) {
        System.out.println("");
    }
}
