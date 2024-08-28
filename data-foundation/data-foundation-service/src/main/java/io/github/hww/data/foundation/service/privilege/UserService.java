package io.github.hww.data.foundation.service.privilege;

import org.springframework.stereotype.Service;

import java.util.ArrayList;


@Service
public class UserService {


    public Iterable<io.github.hww.data.foundation.common.entities.privilege.User> getAllUsers() {
        return new ArrayList<>();
    }
}
