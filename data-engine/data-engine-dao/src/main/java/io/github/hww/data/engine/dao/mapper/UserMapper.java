package io.github.hww.data.engine.dao.mapper;

import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import io.github.hww.data.engine.common.entities.privilege.User;
import org.apache.ibatis.annotations.Mapper;

@Mapper
public interface UserMapper extends BaseMapper<User> {
}
