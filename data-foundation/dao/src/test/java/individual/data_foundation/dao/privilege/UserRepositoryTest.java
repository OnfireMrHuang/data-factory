package individual.data_foundation.dao.privilege;


import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.autoconfigure.domain.EntityScan;
import org.springframework.boot.test.autoconfigure.orm.jpa.DataJpaTest;
import org.springframework.boot.test.autoconfigure.orm.jpa.AutoConfigureDataJpa;
import org.springframework.data.domain.PageRequest;
import org.springframework.data.domain.Sort;
import org.springframework.test.context.jdbc.Sql;
import org.assertj.core.api.WithAssertions;

@DataJpaTest(properties = {
        "spring.datasource.url: jdbc:mysql://localhost:3306/df_config?characterEncoding=utf8&useSSL=false",
        "spring.datasource.username: root",
        "spring.datasource.password: Huang@123",
        "spring.jpa.hibernate.ddl-auto=create-drop",
        "spring.jpa.hibernate.hibernate.dialect=org.hibernate.dialect.MySQL8InnoDBDialect"
})
@AutoConfigureDataJpa
@EntityScan(basePackages = "individual.data_foundation.common.models") // 指定实体类所在的包
public class UserRepositoryTest  implements WithAssertions {
    @Autowired
    private UserRepository userRepository;

    @Test
    @Sql("/v1-0-0/mysql.data.sql")
    public void findAllUsingPage() {
        var users = userRepository.findAll(
                        PageRequest.of(1, 2, Sort.by(Sort.Order.desc("createdTime"))))
                .getContent();

        assertThat(users).extracting("id", "name")
                .contains(tuple("1234567890123456789012", "admin"),
                        tuple("123456789013", "user1"));
    }
}
