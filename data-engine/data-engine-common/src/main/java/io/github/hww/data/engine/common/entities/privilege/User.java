package io.github.hww.data.engine.common.entities.privilege;

import io.github.hww.data.engine.common.entities.base.Identity;
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
