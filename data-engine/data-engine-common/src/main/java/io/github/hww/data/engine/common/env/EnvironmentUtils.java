package io.github.hww.data.engine.common.env;

import org.apache.commons.lang3.StringUtils;

/**
 * 环境变量帮助类
 */
public class EnvironmentUtils {

    public static String getEnv(String name) {
        return System.getenv(name);
    }

    /**
     * 获取环境变量，不存在时，返回枚举中的默认值
     * @param environmentEnum 环境变量配置信息
     * @return 值
     */
    public static String getEnv(EnvironmentEnum environmentEnum) {
        return getEnv(environmentEnum.getPropertyName(),environmentEnum.getDefaultValue());
    }

    /**
     * 获取环境变量 不存在时返回默认值
     *
     * @param name         环境变量信息
     * @param defaultValue 默认值
     * @return 值
     */
    public static String getEnv(String name, String defaultValue) {
        String value = getEnv(name);
        if (StringUtils.isEmpty(value)) {
            value = defaultValue;
        }
        return value;
    }

    /**
     * 设置系统属性默认值 当系统属性名称存在值时，按实际设置，不存在则按照默认值来设置
     *
     * @param name         属性名称
     * @param defaultValue 默认属性值
     */
    public static void setDefaultProperty(String name, String defaultValue) {
        String value = getEnv(name, defaultValue);
        System.setProperty(name, value);
    }

    /**
     * 设置系统属性
     * @param environmentEnum 环境配置枚举
     * @return 返回当前环境配置枚举的实际值
     */
    public static String setProperty(EnvironmentEnum environmentEnum){
        String value = getEnv(environmentEnum);
        System.setProperty(environmentEnum.getPropertyName(), value);
        return value;
    }

    /**
     * 设置系统属性
     * @param environmentEnum 环境配置枚举
     * @return 返回当前环境配置枚举的实际值
     */
    public static String setProperty(EnvironmentEnum environmentEnum,String value){
        System.setProperty(environmentEnum.getPropertyName(), value);
        return value;
    }

    /**
     * 获取对应属性的值
     * @param environmentEnum 环境变量枚举
     * @return 属性的值 
     */
    public static String getProperty(EnvironmentEnum environmentEnum){
        return System.getProperty(environmentEnum.getPropertyName());
    }
}
