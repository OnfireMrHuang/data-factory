package io.github.hww.data.foundation.dao.privilege.database.mapper;

import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import io.github.hww.data.foundation.common.entities.privilege.User;
import org.apache.ibatis.annotations.Mapper;

@Mapper
public interface UserMapper extends BaseMapper<User> {
}
