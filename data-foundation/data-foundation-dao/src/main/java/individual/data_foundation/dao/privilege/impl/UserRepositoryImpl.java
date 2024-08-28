package individual.data_foundation.dao.privilege.impl;

import individual.data_foundation.common.entities.privilege.User;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
public class UserRepositoryImpl implements individual.data_foundation.dao.privilege.UserRepository {
    public List<User> findAll() {
        return null;
    }
}
