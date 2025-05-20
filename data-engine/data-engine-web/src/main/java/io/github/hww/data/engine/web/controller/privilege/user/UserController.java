package io.github.hww.data.engine.web.controller.privilege.user;

import io.github.hww.data.engine.common.entities.privilege.User;
import io.github.hww.data.engine.common.models.web.Response;
import io.github.hww.data.engine.service.privilege.UserService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/api/privilege/user")
public class UserController {

    private final UserService userService;

    // 使用构造函数注入
    @Autowired
    public UserController(UserService userService) {
        this.userService = userService;
    }

    @GetMapping(path="/all")
    public  Response<Iterable<User>> getAllUsers() {
        return Response.success(userService.getAllUsers());
    }
}
