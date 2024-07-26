package individual.data_foundation.web.controller.monitor;

import individual.data_foundation.common.dto.Response;
import individual.data_foundation.common.models.monitor.EnvironmentInfo;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/monitor/environment")
public class Environment {

    @GetMapping("/info")
    public Response<EnvironmentInfo> GetEnvironmentInfo() {
        EnvironmentInfo environmentInfo = EnvironmentInfo.builder().javaVersion("jdk-13").build();
        return Response.success(environmentInfo);
    }
}
