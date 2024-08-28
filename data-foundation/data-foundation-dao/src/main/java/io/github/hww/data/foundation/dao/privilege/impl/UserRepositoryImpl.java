package io.github.hww.data.foundation.dao.privilege.impl;

import io.github.hww.data.foundation.common.entities.privilege.User;
import io.github.hww.data.foundation.dao.privilege.database.mapper.UserMapper;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
public class UserRepositoryImpl implements io.github.hww.data.foundation.dao.privilege.UserRepository {

    private final UserMapper userMapper;

    public UserRepositoryImpl(UserMapper userMapper) {
        this.userMapper = userMapper;
    }

    @Override
    public List<User> findAll() {
        return userMapper.selectList(null);
    }
}
