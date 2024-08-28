package individual.data_foundation.dao.privilege;


import individual.data_foundation.common.entities.privilege.User;

import java.util.List;

public interface UserRepository {
    List<User> findAll();
}
