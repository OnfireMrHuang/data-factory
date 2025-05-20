package io.github.hww.data.engine.common.components.datax.utilities;

import cn.hutool.core.io.FileUtil;
import com.alibaba.datax.common.exception.DataXException;
import com.alibaba.datax.common.util.Configuration;
import com.alibaba.datax.core.util.FrameworkErrorCode;
import com.alibaba.datax.core.util.SecretUtil;
import com.alibaba.datax.core.util.container.CoreConstant;

import java.io.File;
import java.util.*;
import java.util.function.Supplier;

public class ConfigParser {

    private static final String CORE_CONF = "/conf/core.json";

    private static final String PLUGIN_DESC_FILE = "plugin.json";

    public static Configuration parse(final Supplier<String> jobContentCaller) {
        String jobContent = jobContentCaller.get();
        Configuration config = Configuration.from(jobContent);
        Configuration realConfig = SecretUtil.decryptSecretKey(config);
        return parseImpl(realConfig);
    }

    private static Configuration parseImpl(final Configuration configuration) {
        configuration.merge(coreConfig(), false);
        Map<String, String> pluginTypeMap = new HashMap<>();

        //一个reader、一个writer
        String readerName = configuration.getString(CoreConstant.DATAX_JOB_CONTENT_READER_NAME);
        String writerName = configuration.getString(CoreConstant.DATAX_JOB_CONTENT_WRITER_NAME);
        pluginTypeMap.put(readerName, "reader");
        pluginTypeMap.put(writerName, "writer");

        Configuration pluginsDescConfig = parsePluginsConfig(pluginTypeMap);
        configuration.merge(pluginsDescConfig, false);
        return configuration;
    }

    private static Configuration coreConfig() {
        String coreConfigPath = getDataXHome() + CORE_CONF;
        try {
            return Configuration.from(FileUtil.file(coreConfigPath));
        } catch (Exception ignore) {
            throw DataXException.asDataXException("Failed to load the configuration file core.json. " +
                    "Please check whether " + coreConfigPath + " exists!");
        }
    }

    private static String getDataXHome() {
        String dataXHome = System.getenv("DataX_HOME");
        if (dataXHome == null) {
            dataXHome = "/app/dataX";
        }
        return dataXHome;
    }

    private static Configuration parsePluginsConfig(Map<String, String> pluginTypeMap) {
        Configuration configuration = Configuration.newDefault();
        for (File basePackage : runtimeBasePackages(getDataXHome())) {
            if (pluginTypeMap.isEmpty()) {
                break;
            }
            scanPluginByPackage(basePackage, configuration, basePackage.listFiles(), pluginTypeMap);
        }
        if (!pluginTypeMap.isEmpty()) {
            String failedPlugin = pluginTypeMap.keySet().toString();
            String message = "\nplugin %s load failed ：try to analyze the reasons from the following aspects.。\n" +
                    "1: Check if the name of the plugin is spelled correctly, and verify whether DataX supports this plugin\n" +
                    "2：Verify if the <resource></resource> tag has been added under <build></build> section in the pom file of the relevant plugin.\n<resource>" +
                    "                <directory>src/main/resources</directory>\n" +
                    "                <includes>\n" +
                    "                    <include>**/*.*</include>\n" +
                    "                </includes>\n" +
                    "                <filtering>true</filtering>\n" +
                    "            </resource>\n [Refer to the streamreader pom file] \n" +
                    "3: Check that the datax-yourPlugin-example module imported your test plugin";
            message = String.format(message, failedPlugin);
            throw DataXException.asDataXException(FrameworkErrorCode.PLUGIN_INIT_ERROR, message);
        }
        return configuration;
    }

    private static File[] runtimeBasePackages(String path) {
        List<File> basePackages = new ArrayList<>();

        Stack<String> searchPathStack = new Stack<>();
        searchPathStack.add(path);

        while (!searchPathStack.isEmpty()) {
            String currentSearchPath = searchPathStack.pop();
            File[] files = FileUtil.ls(currentSearchPath);
            for (File file : files) {
                if (file.isDirectory()) {
                    basePackages.add(file);
                    searchPathStack.add(file.getPath());
                }
            }
        }
        return basePackages.toArray(new File[0]);
    }

    private static void scanPluginByPackage(File packageFile,
                                            Configuration configuration,
                                            File[] files,
                                            Map<String, String> needPluginTypeMap) {
        if (files == null) {
            return;
        }
        for (File file : files) {
            if (file.isFile() && PLUGIN_DESC_FILE.equals(file.getName())) {
                Configuration pluginDesc = Configuration.from(file);
                String descPluginName = pluginDesc.getString("name", "");

                if (needPluginTypeMap.containsKey(descPluginName)) {

                    String type = needPluginTypeMap.get(descPluginName);
                    configuration.merge(parseOnePlugin(packageFile.getAbsolutePath(), type, descPluginName, pluginDesc), false);
                    needPluginTypeMap.remove(descPluginName);

                }
            } else {
                scanPluginByPackage(packageFile, configuration, file.listFiles(), needPluginTypeMap);
            }
        }
    }

    private static Configuration parseOnePlugin(String packagePath,
                                                String pluginType,
                                                String pluginName,
                                                Configuration pluginDesc) {
        //设置path 兼容jarLoader的加载方式URLClassLoader
        pluginDesc.set("path", packagePath);
        Configuration pluginConfInJob = Configuration.newDefault();
        pluginConfInJob.set(
                String.format("plugin.%s.%s", pluginType, pluginName),
                pluginDesc.getInternal());
        return pluginConfInJob;
    }
}
