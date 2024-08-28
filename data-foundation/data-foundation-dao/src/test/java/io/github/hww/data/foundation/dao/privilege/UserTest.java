package io.github.hww.data.foundation.dao.privilege;

import io.github.hww.data.foundation.common.entities.privilege.User;
import io.github.hww.data.foundation.dao.privilege.impl.UserRepositoryImpl;
import org.junit.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;

import java.util.List;

@SpringBootTest
public class UserTest {

    @Autowired
    private UserRepositoryImpl userRepository;

    @Test
    public void testAllUser() {
        List<User> users = userRepository.findAll();
        for (User user : users) {
            System.out.println(user.toString());
        }
    }
}
