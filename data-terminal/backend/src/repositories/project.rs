use crate::models::project::Project;
use crate::utils::database;
use shaku::{Component, Interface};

pub trait ProjectRepo: Interface {
    fn add_project(&self, project: Project) -> Result<String, sqlx::Error>;
    fn edit_project(&self, project: Project) -> Result<String, sqlx::Error>;
    fn del_project(&self, code: String) -> Result<(), sqlx::Error>;
    fn get_project(&self, code: String) -> Result<Project, sqlx::Error>;
    fn list_project(&self) -> Result<Vec<Project>, sqlx::Error>;
}

#[derive(Component)]
#[shaku(interface = ProjectRepo)]
struct ProjectRepoImpl {}

impl ProjectRepo for ProjectRepoImpl {
    fn add_project(&self, project: Project) -> Result<String, sqlx::Error> {
        let pool = database::get_config_db();
        let code = project.code.clone();

        let rt = tokio::runtime::Runtime::new().expect_err();
        let result = rt.block_on(async {
            let exists = sqlx::query_scalar!(
                "SELECT COUNT(*) FROM project WHERE code = ?",
                code
            )
            .fetch_one(&pool)
            .await;
            return match exists {
                Ok(exists) => exists,
                Err(err) => {
                    return Err(format!("{}", err));
                }
            };
        });



        

        if exists.0 > 0 {
            return Err(format!("项目 code '{}' 已存在", code));
        }

        sqlx::query("INSERT INTO df_c_project (code, name, description, create_status, create_msg) VALUES (?, ?, ?, ?, ?)")
            .bind(&project.code)
            .bind(&project.name)
            .bind(&project.description)
            .bind(project.create_status.to_string())
            .bind(&project.create_msg)
            .execute(&pool)
            .await
            .map_err(|e| format!("插入项目失败: {}", e))?;

        Ok(())
    }

    fn edit_project(&self, project: Project) -> Result<(), String> {}

    fn del_project(&self, code: String) -> Result<(), String> {}

    fn get_project(&self, code: String) -> Result<Project, String> {}

    fn get_projects(&self) -> Result<Vec<Project>, String> {}
}
