package io.github.hww.data.foundation.dao.impl;

import io.github.hww.data.foundation.common.entities.privilege.User;
import io.github.hww.data.foundation.dao.UserRepository;
import io.github.hww.data.foundation.dao.mapper.UserMapper;
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
