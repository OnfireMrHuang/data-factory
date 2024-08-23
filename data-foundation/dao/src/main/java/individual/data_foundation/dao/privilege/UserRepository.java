package individual.data_foundation.dao.privilege;

import org.springframework.data.jpa.repository.JpaRepository;

import org.springframework.stereotype.Repository;
import individual.data_foundation.common.models.privilege.User;

@Repository
public interface UserRepository extends JpaRepository<User, String> {
}
