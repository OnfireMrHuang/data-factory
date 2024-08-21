package individual.data_foundation.dao.privilege;

import org.springframework.data.repository.CrudRepository;
import org.springframework.stereotype.Repository;
import individual.data_foundation.common.models.privilege.User;

@Repository
public interface UserRepository extends CrudRepository<User, String> {
}
