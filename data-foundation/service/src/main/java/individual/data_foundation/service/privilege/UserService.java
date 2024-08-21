package individual.data_foundation.service.privilege;

import individual.data_foundation.common.models.privilege.User;
import individual.data_foundation.dao.privilege.UserRepository;
import org.springframework.stereotype.Service;


@Service
public class UserService {
    private final UserRepository userRepository;

    // 使用构造函数注入
    public UserService(UserRepository userRepository) {
        this.userRepository = userRepository;
    }

    public Iterable<User> getAllUsers() {
        return userRepository.findAll();
    }
}
