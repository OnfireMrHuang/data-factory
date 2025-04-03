package io.github.hww.data.foundation.service.privilege;

import io.github.hww.data.foundation.dao.UserRepository;
import org.springframework.stereotype.Service;

import javax.annotation.Resource;
import java.util.ArrayList;
import io.github.hww.data.foundation.common.entities.privilege.User;


@Service
public class UserService {

    @Resource
    UserRepository userRepository;

    public Iterable<User> getAllUsers() {
        return userRepository.findAll();
    }
}
