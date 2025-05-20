package io.github.hww.data.engine.dao.impl;

import io.github.hww.data.engine.common.entities.privilege.User;
import io.github.hww.data.engine.dao.UserRepository;
import io.github.hww.data.engine.dao.mapper.UserMapper;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
public class UserRepositoryImpl implements UserRepository {

    private final UserMapper userMapper;

    public UserRepositoryImpl(UserMapper userMapper) {
        this.userMapper = userMapper;
    }

    @Override
    public List<User> findAll() {
        return userMapper.selectList(null);
    }
}
