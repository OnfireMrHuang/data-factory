package individual.data_foundation.common.entities.privilege;

import individual.data_foundation.common.entities.base.Identity;
import lombok.Data;
import lombok.EqualsAndHashCode;

@Data
@EqualsAndHashCode(callSuper = true)
public class User extends Identity {
    private String name;
    private String email;
    private String phone;
    private String password;
}
