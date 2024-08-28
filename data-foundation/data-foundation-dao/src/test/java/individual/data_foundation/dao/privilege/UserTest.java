package individual.data_foundation.dao.privilege;

import individual.data_foundation.common.entities.privilege.User;
import org.junit.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;

import java.util.List;

@SpringBootTest
public class UserTest {

    @Autowired
    private UserRepository userRepository;

    @Test
    public void testAllUser() {
        List<User> users = userRepository.findAll();
        for (User user : users) {
            System.out.println(user.toString());
        }
    }
}
