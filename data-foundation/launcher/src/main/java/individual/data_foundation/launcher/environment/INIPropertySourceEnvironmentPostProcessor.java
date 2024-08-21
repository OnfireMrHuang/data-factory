package individual.data_foundation.launcher.environment;

import org.apache.commons.configuration2.INIConfiguration;
import org.apache.commons.configuration2.ex.ConfigurationException;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.env.EnvironmentPostProcessor;
import org.springframework.core.env.ConfigurableEnvironment;
import org.springframework.core.env.PropertiesPropertySource;
import org.springframework.core.io.Resource;
import org.springframework.core.io.support.ResourcePatternResolver;
import org.springframework.core.io.support.ResourcePatternUtils;

import java.io.*;
import java.util.Properties;

public class INIPropertySourceEnvironmentPostProcessor implements EnvironmentPostProcessor {

    @Override
    public void postProcessEnvironment(ConfigurableEnvironment environment, SpringApplication application) {
        try {
            // 使用 ResourcePatternResolver 加载 config.ini 文件
            ResourcePatternResolver resolver = ResourcePatternUtils.getResourcePatternResolver(application.getResourceLoader());
            Resource resource = resolver.getResource("classpath:config.ini");

            // 使用 Apache Commons Configuration 读取 ini 文件
            INIConfiguration iniConfig = new INIConfiguration();
            Reader reader = new InputStreamReader(resource.getInputStream());
            iniConfig.read(reader);

            // 将 ini 配置转换为 Properties 并添加到环境中
            Properties properties = new Properties();
            iniConfig.getSections().forEach(section -> {
                iniConfig.getKeys(section).forEachRemaining(key -> {
                    properties.setProperty(key, iniConfig.getString(key));
                });
            });

            // 将配置添加到propertySource中
            environment.getPropertySources().addLast(new PropertiesPropertySource("customIni", properties));
        } catch (IOException | ConfigurationException e) {
            throw new RuntimeException("Failed to load config.ini", e);
        }
    }
}
