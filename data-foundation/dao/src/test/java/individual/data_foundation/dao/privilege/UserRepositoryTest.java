package individual.data_foundation.dao.privilege;


import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.autoconfigure.domain.EntityScan;
import org.springframework.boot.test.autoconfigure.orm.jpa.DataJpaTest;
import org.springframework.data.domain.PageRequest;
import org.springframework.data.domain.Sort;
import org.springframework.test.context.jdbc.Sql;
import org.springframework.test.context.junit.jupiter.SpringExtension;

import static org.assertj.core.api.Assertions.*;

@DisplayName("Jpa Repository")
@ExtendWith(SpringExtension.class)
@DataJpaTest
@EntityScan(basePackages = "individual.data_foundation.common.models") // 指定实体类所在的包
public class UserRepositoryTest {
    @Autowired
    UserRepository userRepository;

    @DisplayName("Find all using page")
    @Test
    @Sql("/v1-0-0/mysql.data.sql")
    void findAllUsingPage() {
        var users = userRepository.findAll(
                        PageRequest.of(1, 2, Sort.by(Sort.Order.desc("createdTime"))))
                .getContent();

        assertThat(users).extracting("id", "name")
                .contains(tuple("1234567890123456789012", "admin"),
                        tuple("123456789013", "user1"));
    }
}
