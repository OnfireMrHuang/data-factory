package io.github.hww.data.engine.dao;


import io.github.hww.data.engine.common.entities.privilege.User;

import java.util.List;

public interface UserRepository {
    List<User> findAll();
}
