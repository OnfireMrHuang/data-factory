package individual.data_foundation.dao.privilege;

import org.springframework.data.repository.CrudRepository;
import individual.data_foundation.common.models.privilege.User;
import org.springframework.stereotype.Repository;

@Repository
public interface UserRepository extends CrudRepository<User, String> {
}
