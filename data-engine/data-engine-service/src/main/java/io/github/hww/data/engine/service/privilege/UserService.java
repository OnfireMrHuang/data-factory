package io.github.hww.data.engine.service.privilege;

import io.github.hww.data.engine.dao.UserRepository;
import org.springframework.stereotype.Service;

import javax.annotation.Resource;
import java.util.ArrayList;
import io.github.hww.data.engine.common.entities.privilege.User;


@Service
public class UserService {

    @Resource
    UserRepository userRepository;

    public Iterable<User> getAllUsers() {
        return userRepository.findAll();
    }
}
