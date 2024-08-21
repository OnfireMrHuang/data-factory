package individual.data_foundation.common.models.privilege;

import individual.data_foundation.common.models.base.Identity;
import jakarta.persistence.Entity;
import jakarta.persistence.Table;
import lombok.Data;
import lombok.EqualsAndHashCode;

@Data
@EqualsAndHashCode(callSuper = true)
@Entity
@Table(name = "user")
public class User extends Identity {
    private String name;
    private String email;
    private String phone;
    private String password;
}
