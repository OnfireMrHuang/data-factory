use crate::models::collection::{CollectType, CollectionCategory, CollectTask};
use crate::models::Error;
use crate::services::collection::CollectionServiceImpl;

pub async fn generate_test_run_steps(category: CollectionCategory, collect_type: CollectType) -> Result<Vec<(String, String)>, Error> {
    match (category, collect_type) {
        (CollectionCategory::Database, CollectType::Incremental) => {
            let steps = vec![
                ("数据库连接检查".to_string(), "检查数据库是否可连接、权限是否正确配置".to_string()),
                ("目标表DDL检查".to_string(), "检查是否可执行".to_string()),
                ("清洗SQL检查".to_string(), "检查SQL是否可执行、与目标表字段是否一致等等".to_string()),
                ("数据预览".to_string(), "预览数据, 检查数据类型及内容是否符合预期".to_string()),
            ];
            return Ok(steps);
        }
        (CollectionCategory::Database, CollectType::Full) => {
            let steps = vec![
                ("数据库连接检查".to_string(), "检查数据库是否可连接、权限是否正确配置".to_string()),
                ("目标表DDL检查".to_string(), "检查是否可执行".to_string()),
                ("行更新规则清洗SQL检查".to_string(), "检查SQL是否可执行、与目标表字段是否一致等等".to_string()),
                ("行更新规则监听SQL检查".to_string(), "检查SQL格式是否正确、是否可执行".to_string()),
                ("字段更新规则清洗SQL检查".to_string(), "检查SQL是否可执行, 更新字段是否正确".to_string()),
                ("字段更新规则监听SQL检查".to_string(), "检查SQL格式是否正确、是否可执行".to_string()),
                ("数据预览".to_string(), "预览数据, 检查数据类型及内容是否符合预期".to_string()),
            ];
            return Ok(steps);
        }
        _ => Err(Error::InvalidValue("Test run for given collection type and category is not supported".to_string()))
    }
}




impl CollectionServiceImpl {
    pub async fn execute_test_run(&self, project_code: String, collection_task: CollectTask) { 
        // First, 



    }
}
