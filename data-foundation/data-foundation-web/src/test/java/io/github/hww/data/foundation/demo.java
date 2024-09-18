package io.github.hww.data.foundation;

import io.github.hww.data.foundation.common.entities.privilege.User;
import io.github.hww.data.foundation.web.controller.privilege.UserController;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import org.testng.annotations.Test;


@SpringBootTest
public class demo {

    @Autowired
    private UserController userController;

    @Test
    public void test() {
        Iterable<User> users = userController.getAllUsers();
        for (User user : users) {
            System.out.println(user.toString());
        }
    }
}
