package individual.data_foundation.service.privilege;

import org.springframework.stereotype.Service;

import java.util.ArrayList;


@Service
public class UserService {


    public Iterable<individual.data_foundation.common.entities.privilege.User> getAllUsers() {
        return new ArrayList<>();
    }
}
