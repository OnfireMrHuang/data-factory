package individual.data_foundation.common.exception;

/**
 * 返回到传输层的异常接口，Service层定义的异常信息实现此接口
 *
 * @author Harry.Williams
 * @date 2024.07.25
 */
public interface IResponse {

    /**
     * 代码，例如异常码，不同异常信息应使用不同的异常码
     *
     * @return 代码
     */
    Integer code();

    /**
     * 信息， 例如异常信息
     *
     * @return message
     */
    String message();

    /**
     * 获取国际化key
     *
     * @return 国际化key
     */
    default String i18nKey() {
        return "response.message." + code();
    }
}
