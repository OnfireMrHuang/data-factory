package io.github.hww.data.foundation.common.entities.base;


import lombok.Data;


@Data
public abstract class IdentityWithOperator extends Identity {
    private String createdBy;
    private String updatedBy;
}
